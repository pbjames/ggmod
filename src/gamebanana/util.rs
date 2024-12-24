use regex::Regex;

fn to_snake_case(s: String) -> String {
    let mut new = String::new();
    let mut last: char = '"';
    for char in s.chars() {
        if last == '"' {
            new.push(char.to_ascii_lowercase());
        } else if char.is_uppercase() {
            new.push('_');
            new.push(char.to_ascii_lowercase());
        } else {
            new.push(char);
        }
        last = char;
    }
    new
}

pub fn to_human(s: &str) -> Result<String, regex::Error> {
    let pattern = r"(_[a-z]+)(([A-Z][a-z]+)+)";
    let re = Regex::new(pattern).unwrap();
    let res = re.replace_all(s, "$2");
    let res = to_snake_case(res.into_owned());
    Ok(res.clone())
}

#[cfg(test)]
mod test {
    use crate::gamebanana::{to_human, util::to_snake_case};

    #[test]
    fn snake_case_works() {
        assert_eq!(to_snake_case(String::from("DateAdded")), "date_added");
        assert_eq!(
            to_snake_case(String::from("SuperLongname1Name2")),
            "super_longname1_name2"
        );
    }

    #[test]
    fn human_works() {
        let before = "\
        \"_tsDateUpdated\": 1722410377,\
        \"_tsDateAdded\": 1624842936,\
        \"_sModelName\": \"Mod\",\
        \"_sName\": \"Potato Mod Lite\",\
        \"_bIsNsfw\": false,\
        \"_aPreviewMedia\": [],\
        \"_nDownloadCount\": 183556,\
        \"_sDescription\": \"\",\
        \"_nViewCount\": 278976,\
        \"_nLikeCount\": 119,\
        \"_aModManagerIntegrations\": {},\
        \"_aAlternateFileSources\": []";
        let after = "\"date_updated\": 1722410377,\"date_added\": 1624842936,\"model_name\": \"mod\",\"name\": \"potato _mod _lite\",\"is_nsfw\": false,\"preview_media\": [],\"download_count\": 183556,\"description\": \"\",\"view_count\": 278976,\"like_count\": 119,\"mod_manager_integrations\": {},\"alternate_file_sources\": []";
        assert_eq!(to_human(before).unwrap(), after);
    }
}
