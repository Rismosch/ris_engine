use std::io::SeekFrom;
use std::path::Path;
use std::path::PathBuf;

use ris_error::RisResult;
use ris_file::io::FatPtr;

use crate::ExplanationLevel;
use crate::ICommand;


const ORG_NAME: &str = "Rismosch";
const APP_NAME: &str = "ris_engine";
const PROFILER: &str = "profiler";

const SEPARATOR: char = ';';

const KEYS: &[&str] = &[
    "parent",
    "id",
    "generation",
    "file",
    "line",
    "min",
    "max",
    "sum",
    "average",
    "median",
    "percentage",
];

#[derive(Debug)]
struct ParsedCsvLine {
    parent: String,
    json: String,
}

pub struct ProfilerHtml;

impl ICommand for ProfilerHtml {
    fn args() -> String {
        String::new()
    }

    fn explanation(_level: ExplanationLevel) -> String {
        String::from("Renders profiler results as an Html.")
    }

    fn run(_args: Vec<String>, target_dir: PathBuf) -> RisResult<()> {
        let chart_js_path = crate::util::get_root_dir()?
            .join("external")
            .join("javascript")
            .join("Chart.js");
        eprintln!("reading... {:?}", chart_js_path);
        let chart_js = read_text_file(chart_js_path)?;


        let pref_path = sdl2::filesystem::pref_path(ORG_NAME, APP_NAME)?;
        let profiler_dir = PathBuf::from(pref_path).join(PROFILER);
        eprintln!("reading... {:?}", profiler_dir);

        let mut parsed_csv_files = Vec::new();

        for (i, entry) in profiler_dir.read_dir()?.enumerate() {
            let entry = entry?;
            let path = entry.path();
            let metadata = entry.metadata()?;

            let entry_is_file = metadata.is_file();
            let path_ends_with_csv = path
                .extension()
                .map(|x| x.to_ascii_lowercase() == "csv")
                .unwrap_or(false);

            if !entry_is_file || !path_ends_with_csv {
                eprintln!("cannot read {:?}", path);
                continue;
            }

            eprintln!("reading... {:?}", path);
            let file_name = match path.file_name().map(|x| x.to_str()) {
                Some(Some(file_name)) => file_name.to_string(),
                _ => format!("csv {}", i),
            };

            eprintln!("parse csv... {:?}", path);
            let csv = read_text_file(&path)?;
            let mut lines = csv.lines();

            let Some(first_line) = lines.next() else {
                eprintln!("csv contains no lines");
                continue;
            };

            let headers = first_line
                .split(SEPARATOR)
                .collect::<Vec<_>>();
            if let Err(e) = ris_error::assert!(headers.len() == KEYS.len()) {
                eprintln!("failed to parse header: {}", e);
                continue;
            }

            for (i, &header) in headers.iter().enumerate() {
                let key = KEYS[i];
                if header != key {
                    eprintln!(
                        "expected header {} to be \"{}\" but was \"{}\"",
                        i, key, header,
                    );
                    continue;
                }
            }

            // header is as expected
            let mut failed_to_parse = false;
            
            let mut parsed_csv_lines = Vec::new();
            for (i, line) in lines.skip(1).enumerate() {
                let parsed = match parse_csv_line(line) {
                    Ok(parsed) => parsed,
                    Err(e) => {
                        eprintln!("failed to parse line {}: {}", i + 1, e);
                        failed_to_parse = true;
                        break;
                    },
                };

                let position = parsed_csv_lines
                    .iter()
                    .position(|(x, _)| *x == parsed.parent);

                match position {
                    None => parsed_csv_lines.push((parsed.parent.clone(), vec![parsed])),
                    Some(i) => parsed_csv_lines[i].1.push(parsed),
                }
            }

            if failed_to_parse {
                continue;
            }

            parsed_csv_files.push((file_name, parsed_csv_lines));
            eprintln!("success parsing!");
        }

        parsed_csv_files.sort_by(|left, right| left.0.cmp(&right.0));

        eprintln!("generating html...");

        let mut html = String::new();
        html.push_str(&format!("
<!DOCTYPE html>
<html>
<script>
{}
</script>
<body>

<select id=\"csv_file\" onchange=\"csv_file_changed()\">", chart_js));

        for (file_name, _) in parsed_csv_files.iter() {
            html.push_str(&format!("
<option>{}</option>", file_name));
        }

        html.push_str(&format!("
</select>

<canvas id=\"myChart\" style=\"width:100%;max-width:600px\"></canvas>

<script>

var csv_data = {{"));
        
        for (file_name, parsed_csv_lines) in parsed_csv_files.iter() {
            html.push_str(&format!("
    {}: {{", sanitize_key(file_name)));

            for (parent, lines) in parsed_csv_lines.iter() {
                html.push_str(&format!("
        {}: [", sanitize_key(parent)));

                 for line in lines.iter() {
                     html.push_str(&format!("
             {},", line.json));
                 }

                html.push_str(&format!("
        ],"));
            }

            html.push_str(&format!("
    }},"));
        }

        html.push_str(&format!("
}};

console.log(csv_data);

var csv_file_select = document.getElementById(\"csv_file\");
csv_file_changed();
function csv_file_changed() {{
    var value = csv_file_select.value;
    console.log(value);
}}

const xValues = [\"Italy\", \"France\", \"Spain\", \"USA\", \"Argentina\"];
const yValues = [55, 49, 44, 24, 15];
const barColors = [\"red\", \"green\",\"blue\",\"orange\",\"brown\"];

new Chart(\"myChart\", {{
    type: \"bar\",
    data: {{
        labels: xValues,
        datasets: [
            {{
                backgroundColor: barColors,
                data: yValues
            }},
            {{
                backgroundColor: barColors,
                data: yValues
            }},
        ]
    }},
    options: {{
        legend: {{display: false}},
        title: {{
            display: true,
            text: \"World Wine Production 2018\"
        }}
    }}
}});
</script>

</body>
</html>"));

        eprintln!("writing html...");
        ris_file::util::clean_or_create_dir(&target_dir)?;
        let dst_path = PathBuf::from(&target_dir).join("index.html");
        let mut file = std::fs::File::create(&dst_path)?;
        ris_file::io::write_checked(&mut file, html.as_bytes())?;

        eprintln!("done! resulting html can be found in {:?}", dst_path);

        Ok(())
    }
}

fn read_text_file(path: impl AsRef<Path>) -> RisResult<String> {
    let mut file = std::fs::File::open(path)?;
    let len = ris_file::io::seek(&mut file, SeekFrom::End(0))?;

    let fatptr = FatPtr::begin_end(0, len)?;
    let data = ris_file::io::read_unsized(&mut file, fatptr)?;
    let text = String::from_utf8(data)?;

    Ok(text)
}

fn parse_csv_line(line: &str) -> RisResult<ParsedCsvLine> {
    let cells = line
        .split(SEPARATOR)
        .collect::<Vec<_>>();
    ris_error::assert!(cells.len() == KEYS.len())?;

    let mut json = String::from("{ ");
    json.push_str(&format!("{}: \"{}\"", KEYS[0], cells[0]));
    json.push_str(", ");
    json.push_str(&format!("{}: \"{}\"", KEYS[1], cells[1]));
    json.push_str(", ");
    json.push_str(&format!("{}: {}", KEYS[2], cells[2]));
    json.push_str(", ");
    json.push_str(&format!("{}: \"{}\"", KEYS[3], cells[3]));
    json.push_str(", ");
    json.push_str(&format!("{}: {}", KEYS[4], cells[4]));
    json.push_str(", ");
    json.push_str(&format!("{}: {}", KEYS[5], cells[5]));
    json.push_str(", ");
    json.push_str(&format!("{}: {}", KEYS[6], cells[6]));
    json.push_str(", ");
    json.push_str(&format!("{}: {}", KEYS[7], cells[7]));
    json.push_str(", ");
    json.push_str(&format!("{}: {}", KEYS[8], cells[8]));
    json.push_str(", ");
    json.push_str(&format!("{}: {}", KEYS[9], cells[9]));
    json.push_str(", ");
    json.push_str(&format!("{}: {}", KEYS[10], cells[10]));
    json.push_str(" }");

    Ok(ParsedCsvLine{
        parent: cells[0].to_string(),
        json,
    })
}

fn sanitize_key(key: &str) -> String {
    format!(
        "_{}",
        key
            .replace('+', "_")
            .replace('-', "_")
            .replace('.', "_")
            .replace(' ', "_")
            .replace('(', "_")
            .replace(')', "_")
    )
}
