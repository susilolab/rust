use std::fs;
use std::path::Path;
use std::env;

fn main() {
    let args = env::args().nth(1).unwrap();

    let file_names = [
        "protected/modules/test/controllers/SessionController.php",
        "protected/components/system/AppConfigBehavior.php",
        "protected/config/console.php",
        "protected/config/production.php",
        "protected/extensions/dbparam/XDbParam.php",
        "protected/sweeto/Sweeto.php",
        "protected/yiic.php",
        "protected/components/system/Utility.php",
        "protected/models/swt/Debuging.php",
        "protected/modules/member/models/CcnJobseekerBio.php",
        "protected/modules/srbac/models/AuthItem.php",
        "protected/modules/test/models/OsSessionBidang.php",
        "protected/modules/test/models/OsSessionTes.php",
        "protected/modules/test/models/OsSessionUser.php",
        "protected/modules/test/models/RecruitmentSessionsSource.php",        
    ];

    let _ = fs::create_dir("backup");
    for name in file_names.iter() {
        let path = Path::new(name);
        let dst = format!("backup/{}", path.parent().unwrap().to_str().unwrap());
        let file_name = path.file_name().unwrap().to_str().unwrap();
        
        if !Path::new(&dst).exists() {
            let _ = fs::create_dir_all(&dst);
        }

        let _ = fs::copy(
            format!("{}/{}", args, name),
            format!("{}/{}", dst, file_name)
        );
        println!("Menyalin file '{}'..", file_name);
        // println!("{}", path.file_name().unwrap().to_str().unwrap());
        // println!("{}", path.parent().unwrap().to_str().unwrap());
    }
}