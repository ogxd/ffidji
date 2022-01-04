use ffidji;

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::process::Command;
    use rstest::rstest;

    use super::*;

    #[rstest]
    //#[case("tests/interface_empty.xml")]
    #[case("tests/interface_structs.xml")]
    //#[case("tests/interface_arrays.xml")]
    //#[case("tests/interface_strings.xml")]
    fn test_generates_and_builds(#[case] interface: &str) {
        println!("current dir: {:?}", std::env::current_dir());

        let opts = ffidji::Opts {
            from_lang: "csharp".to_owned(),
            from_output_path: Some(PathBuf::from("tests/csharp/interface.cs".to_owned())),
            to_lang: "rust".to_owned(),
            to_output_path: Some(PathBuf::from("tests/rust/interface.rs".to_owned())),
            interface_path: Some(PathBuf::from(interface.to_owned())),
        };

        ffidji::execute(&opts);

        let status = Command::new("dotnet")
            .arg("build")
            .arg("tests/csharp/Test.csproj")
            .arg("-o")
            .arg("tests/csharp/bin")
            .status()
            .expect("failed to execute process");

        assert!(status.success());
    }
}