// Copyright 2015 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

use std::error::Error;

/// Global Singleton Client
pub struct Client {
    client: ::std::sync::Arc<::std::sync::Mutex<::safe_client::client::Client>>,
}

/// Getter for the Singleton
pub fn get_test_client() -> ::std::sync::Arc<::std::sync::Mutex<::safe_client::client::Client>> {
    static mut CLIENT: *const Client = 0 as *const Client;
    static mut ONCE: ::std::sync::Once = ::std::sync::ONCE_INIT;

    unsafe {
        ONCE.call_once(|| {
            CLIENT = ::std::mem::transmute(Box::new(
                    Client {
                        client: ::std::sync::Arc::new(::std::sync::Mutex::new(::safe_client::utility::test_utils::get_client().unwrap())), // TODO implement clone for errors and remove unwrap
                    }
                    ));
        });

        (*CLIENT).client.clone()
    }
}

/// Tokenise the given path
pub fn path_tokeniser(cstr_path: &::std::ffi::CStr) -> Result<Vec<String>, ::errors::FfiError> {
    let string_path = try!(String::from_utf8(cstr_path.to_bytes().iter().map(|a| *a).collect()).map_err(|error| ::errors::FfiError::from(error.description())));
    Ok(string_path.split("/").filter(|a| !a.is_empty()).map(|a| a.to_string()).collect())
}

pub fn get_final_subdirectory(tokens: &Vec<String>) -> Result<::safe_nfs::directory_listing::DirectoryListing, ::errors::FfiError> {
    let dir_helper = ::safe_nfs::helper::directory_helper::DirectoryHelper::new(get_test_client());
    let mut current_dir_listing = try!(dir_helper.get_user_root_directory_listing());
    for it in tokens.iter() {
        let current_dir_info = try!(current_dir_listing.get_sub_directories().iter().find(|a| *a.get_name() == *it).ok_or(::errors::FfiError::PathNotFound)).clone();
        current_dir_listing = try!(dir_helper.get(current_dir_info.get_key(),
                                                  current_dir_info.get_metadata().is_versioned(),
                                                  current_dir_info.get_metadata().get_access_level()));
    }

    Ok(current_dir_listing)
}

pub fn get_file_size(name: &String, parent_directory: &::safe_nfs::directory_listing::DirectoryListing) -> Result<u64, ::errors::FfiError> {
    let reader = try!(get_reader(name, parent_directory));
    Ok(reader.size())
}

pub fn get_file_content(name: &String, parent_directory: &::safe_nfs::directory_listing::DirectoryListing) -> Result<Vec<u8>, ::errors::FfiError> {
    let mut reader = try!(get_reader(name, parent_directory));
    let size = reader.size();
    Ok(try!(reader.read(0, size)))
}

fn get_reader<'a>(name: &String, parent_directory: &'a ::safe_nfs::directory_listing::DirectoryListing) -> Result<::safe_nfs::helper::reader::Reader<'a>, ::errors::FfiError> {
    let file = try!(parent_directory.get_files().iter().find(|a| *a.get_name() == *name).ok_or(::errors::FfiError::FileNotFound));
    let file_helper = ::safe_nfs::helper::file_helper::FileHelper::new(get_test_client());
    Ok(file_helper.read(file))
}

#[cfg(test)]
mod test {
    use super::*;
    use std::error::Error;

    #[test]
    fn parse_path() {
        let path_0 = eval_result!(::std::ffi::CString::new("/abc/d/ef").map_err(|error| ::errors::FfiError::from(error.description())));
        let path_1 = eval_result!(::std::ffi::CString::new("/abc/d/ef/").map_err(|error| ::errors::FfiError::from(error.description())));
        let path_2 = eval_result!(::std::ffi::CString::new("///abc///d/ef////").map_err(|error| ::errors::FfiError::from(error.description())));

        let expected = vec!["abc".to_string(),
                            "d".to_string(),
                            "ef".to_string()];

        let tokenised_0 = eval_result!(path_tokeniser(&path_0));
        let tokenised_1 = eval_result!(path_tokeniser(&path_1));
        let tokenised_2 = eval_result!(path_tokeniser(&path_2));

        assert_eq!(tokenised_0, expected);
        assert_eq!(tokenised_1, expected);
        assert_eq!(tokenised_2, expected);
    }
}
