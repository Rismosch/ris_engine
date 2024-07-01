use std::io::SeekFrom;
use std::path::Path;
use std::path::PathBuf;

use ris_error::RisResult;
use ris_file::io::FatPtr;

use crate::ExplanationLevel;
use crate::ICommand;

pub struct ProfilerHtml;

impl ICommand for ProfilerHtml {
    fn args() -> String {
        String::new()
    }

    fn explanation(_level: ExplanationLevel) -> String {
        String::from("Renders profiler results as an Html.")
    }

    fn run(_args: Vec<String>, target_dir: PathBuf) -> RisResult<()> {
        let pref_path = sdl2::filesystem::pref_path("Rismosch", "ris_engine")?;
        let profiler_dir = PathBuf::from(pref_path).join("profiler");

        let chart_js_path = crate::util::get_root_dir()?
            .join("cli")
            .join("javascript")
            .join("Chart.js");
        eprintln!("reading... {:?}", chart_js_path);
        let chart_js = read_text_file(chart_js_path)?;

        eprintln!("generating html...");

        let html = format!("
<!DOCTYPE html>
<html>
<script>
{}
</script>
<body>

<canvas id=\"myChart\" style=\"width:100%;max-width:600px\"></canvas>

<script>
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
</html>
",
            chart_js
        );


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

