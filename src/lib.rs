use std::collections::BTreeMap;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct WptReport {
    run_info: serde_json::Value,
    time_start: u64,
    time_end: u64,
    results: Vec<TestResult>,
    lsan_leaks: Option<Vec<LsanLeak>>,
    mozleak: Option<BTreeMap<String, MozLeak>>,
}

#[derive(Serialize, Deserialize)]
struct TestResult {
    test: String,
    subtests: Vec<SubtestResult>,
    status: TestStatus,
    expected: Option<TestStatus>,
    known_intermittent: Option<Vec<TestStatus>>,
    message: Option<String>,
    duration: Option<i64>,
    asserts: Option<AssertionCount>,
    reftest_screenshots: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
struct SubtestResult {
    name: String,
    status: SubtestStatus,
    expected: Option<SubtestStatus>,
    known_intermittent: Option<Vec<SubtestStatus>>,
    message: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct AssertionCount {
    count: u32,
    min: u32,
    max: u32,
}

#[derive(Serialize, Deserialize)]
struct LsanLeak {
    frames: Vec<String>,
    scope: String,
    allowed_match: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct MozLeak {
    objects: Vec<MozLeakObject>,
    total: Vec<MozLeakTotal>,
}

#[derive(Serialize, Deserialize)]
struct MozLeakObject {
    process: Option<String>,
    name: String,
    allowed: bool,
    bytes: u64,
}

#[derive(Serialize, Deserialize)]
struct MozLeakTotal {
    bytes: u64,
    threshold: u64,
    process: Option<String>,
}

#[derive(Serialize, Deserialize)]
enum TestStatus {
    PASS,
    FAIL,
    OK,
    ERROR,
    TIMEOUT,
    CRASH,
    ASSERT,
    SKIP,
}

#[derive(Serialize, Deserialize)]
enum SubtestStatus {
    PASS,
    FAIL,
    ERROR,
    TIMEOUT,
    ASSERT,
    NOTRUN,
    SKIP,
}

#[cfg(test)]
mod tests {
    use super::WptReport;
    use serde_json;
    use std::fs;
    use std::io::Read;
    use std::path::PathBuf;

    fn get_data_paths() -> Vec<PathBuf> {
        let mut data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        data_dir.push("testdata");
        println!("{:?}", data_dir);
        let mut paths = Vec::new();
        for entry in fs::read_dir(data_dir).unwrap() {
            let path = entry.unwrap().path();
            println!("{:?}", path);
            if path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with("wptreport.json")
            {
                paths.push(path);
            }
        }
        paths
    }

    #[test]
    fn parse_examples() {
        for path in get_data_paths() {
            println!("{:?}", path);
            let mut buf = String::new();
            let mut f = fs::File::open(path).unwrap();
            f.read_to_string(&mut buf).unwrap();
            let _: WptReport = serde_json::from_str(&buf).unwrap();
        }
    }
}
