use std::io::SeekFrom;
use std::path::Path;
use std::path::PathBuf;

use ris_error::RisResult;
use ris_io::FatPtr;

use super::ExplanationLevel;
use super::ICommand;
use super::util;

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
    fn name(&self) -> String {
        "profiler_html".to_string()
    }

    fn args(&self) -> String {
        String::new()
    }

    fn explanation(&self, _level: ExplanationLevel) -> String {
        String::from("Renders profiler results as an Html.")
    }

    fn run(&self, _args: Vec<String>, target_dir: &Path) -> RisResult<()> {
        let chart_js_path = util::get_root_dir()?
            .join("third_party")
            .join("Chart.js")
            .join("dist")
            .join("Chart.js");
        eprintln!("reading... \"{}\"", chart_js_path.display(),);
        let chart_js = read_text_file(chart_js_path)?;

        let pref_path = sdl2::filesystem::pref_path(ORG_NAME, APP_NAME)?;
        let profiler_dir = PathBuf::from(pref_path).join(PROFILER);
        eprintln!("reading... \"{}\"", profiler_dir.display());

        let mut parsed_csv_files = Vec::new();

        for (i, entry) in profiler_dir.read_dir()?.enumerate() {
            let entry = entry?;
            let path = entry.path();
            let metadata = entry.metadata()?;

            let entry_is_file = metadata.is_file();
            let path_ends_with_csv = path
                .extension()
                .map(|x| x.eq_ignore_ascii_case("csv"))
                .unwrap_or(false);

            if !entry_is_file || !path_ends_with_csv {
                eprintln!("cannot read \"{}\"", path.display());
                continue;
            }

            eprintln!("reading... \"{}\"", path.display());
            let file_name = match path.file_name().map(|x| x.to_str()) {
                Some(Some(file_name)) => file_name.to_string(),
                _ => format!("csv {}", i),
            };

            eprintln!("parse csv... \"{}\"", path.display());
            let csv = read_text_file(&path)?;
            let mut lines = csv.lines();

            let Some(first_line) = lines.next() else {
                eprintln!("csv contains no lines");
                continue;
            };

            let headers = first_line.split(SEPARATOR).collect::<Vec<_>>();
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
                    }
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
        html.push_str(&format!(
            "
<!DOCTYPE html>
<html>
<script>
{}
</script>
<body style=\"background: lightgray;\">

<select id=\"csv_file\" onchange=\"csv_file_changed()\">
",
            chart_js
        ));

        for (file_name, _) in parsed_csv_files.iter() {
            html.push_str(&format!(
                "
<option>{}</option>",
                file_name
            ));
        }

        html.push_str(
            "
</select>

<select id=\"parent\" onchange=\"render_chart()\"></select>

<br>

<input id=\"min\" type=\"checkbox\" onchange=\"render_chart()\" checked><label>min</label>
<input id=\"max\" type=\"checkbox\" onchange=\"render_chart()\" checked><label>max</label>
<input id=\"sum\" type=\"checkbox\" onchange=\"render_chart()\"><label>sum</label>
<input id=\"average\" type=\"checkbox\" onchange=\"render_chart()\" checked><label>average</label>
<input id=\"median\" type=\"checkbox\" onchange=\"render_chart()\" checked><label>median</label>
<input id=\"percentage\" type=\"checkbox\" onchange=\"render_chart()\"><label>percentage</label>

<br>

<canvas id=\"myChart\" style=\"width:100%;max-width:600px\"></canvas>

<script>

var csv_data = {",
        );

        for (file_name, parsed_csv_lines) in parsed_csv_files.iter() {
            html.push_str(&format!(
                "
    {}: {{",
                sanitize_key(file_name)
            ));

            for (parent, lines) in parsed_csv_lines.iter() {
                html.push_str(&format!(
                    "
        {}: [",
                    sanitize_key(parent)
                ));

                for line in lines.iter() {
                    html.push_str(&format!(
                        "
             {},",
                        line.json
                    ));
                }

                html.push_str(
                    "
        ],",
                );
            }

            html.push_str(
                "
    },",
            );
        }

        html.push_str(
            "
};

function to_key(value) {
    let result = value;
    result = result.replaceAll(\"+\", \"_\");
    result = result.replaceAll(\"-\", \"_\");
    result = result.replaceAll(\".\", \"_\");
    result = result.replaceAll(\" \", \"_\");
    result = result.replaceAll(\"(\", \"_\");
    result = result.replaceAll(\")\", \"_\");
    return \"_\" + result;
}

var csv_file_select = document.getElementById(\"csv_file\");
var parent_select = document.getElementById(\"parent\");
var min_checkbox = document.getElementById(\"min\");
var max_checkbox = document.getElementById(\"max\");
var sum_checkbox = document.getElementById(\"sum\");
var average_checkbox = document.getElementById(\"average\");
var median_checkbox = document.getElementById(\"median\");
var percentage_checkbox = document.getElementById(\"percentage\");

csv_file_changed();
function csv_file_changed() {
    let value = csv_file_select.value;
    let key = to_key(value);
    let csv = csv_data[key];

    let parentInnerHTML = \"\";

    for (var parent_key in csv) {
        let csv_lines = csv[parent_key];

        let parent
        if (csv_lines.length > 0) {
            parent = csv_lines[0][\"parent\"];
        } else {
            parent = parent_key;
        }

        parentInnerHTML += \"<option>\" + parent + \"</option>\";
    }

    parent_select.innerHTML = parentInnerHTML;
    render_chart();
}

var chart;

function render_chart() {
    let csv_value = csv_file_select.value;
    let parent_value = parent_select.value;
    let csv_key = to_key(csv_value);
    let parent_key = to_key(parent_value);
    let csv_lines = csv_data[csv_key][parent_key];

    let labels = [];
    for (var i = 0; i < csv_lines.length; ++i) {
        let line = csv_lines[i];
        labels.push(line[\"id\"]);
    }

    let datasets = [];

    function push_data_set(checkbox, key, color) {
        if (!checkbox.checked) {
            return;
        }

        let colors = [];
        let values = [];
        for (var i = 0; i < csv_lines.length; ++i) {
            let line = csv_lines[i];
            let value = line[key];

            colors.push(color);
            values.push(value);
        }

        let dataset = {
            label: key,
            backgroundColor: colors,
            data: values,
        };
        datasets.push(dataset);
    }

    push_data_set(min_checkbox, \"min\", \"red\");
    push_data_set(max_checkbox, \"max\", \"yellow\");
    push_data_set(sum_checkbox, \"sum\", \"green\");
    push_data_set(average_checkbox, \"average\", \"cyan\");
    push_data_set(median_checkbox, \"median\", \"blue\");
    push_data_set(percentage_checkbox, \"percentage\", \"magenta\");

    let data = {
        labels: labels,
        datasets: datasets,
    };

    if (chart) {
        chart.data = data;
        chart.update();
    } else {
        chart = new Chart(\"myChart\", {
            type: \"bar\",
            data: data,
            options: {
                legend: {
                    display: true,
                },
                title: {
                    display: true,
                    text: \"Profiler Evaluation\"
                }
            }
        });
    }
}

</script>

</body>
</html>",
        );

        eprintln!("writing html...");
        ris_io::util::clean_or_create_dir(target_dir)?;
        let dst_path = PathBuf::from(target_dir).join("index.html");
        let mut file = std::fs::File::create(&dst_path)?;
        ris_io::write(&mut file, html.as_bytes())?;

        eprintln!(
            "done! resulting html can be found in \"{}\"",
            dst_path.display(),
        );

        Ok(())
    }
}

fn read_text_file(path: impl AsRef<Path>) -> RisResult<String> {
    let mut file = std::fs::File::open(path)?;
    let len = ris_io::seek(&mut file, SeekFrom::End(0))?;

    let fatptr = FatPtr::begin_end(0, len)?;
    let data = ris_io::read_at(&mut file, fatptr)?;
    let text = String::from_utf8(data)?;

    Ok(text)
}

fn parse_csv_line(line: &str) -> RisResult<ParsedCsvLine> {
    let cells = line.split(SEPARATOR).collect::<Vec<_>>();
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

    Ok(ParsedCsvLine {
        parent: cells[0].to_string(),
        json,
    })
}

fn sanitize_key(key: &str) -> String {
    format!("_{}", key.replace(['+', '-', '.', ' ', '(', ')'], "_"))
}
