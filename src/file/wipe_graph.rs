use plotters::prelude::*;

pub struct WipeGraph;
impl WipeGraph {
    pub fn new() -> Self {
        return Self;
    }

    pub fn create_graph(&self, data: &Vec<u8>, area_name: &str) -> anyhow::Result<String> {
        if data.is_empty() {
            return Err(anyhow::anyhow!("配列が見つかりません。"));
        }
        const FILE_NAME: &str = "graph.png";
        let root = BitMapBackend::new(FILE_NAME, (640, 480)).into_drawing_area();

        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .margin(5)
            .caption(area_name, ("meiryo", 40))
            .build_cartesian_2d(
                (1u32..*data.iter().max().unwrap() as u32).into_segmented(),
                0u32..data.len() as u32,
            )?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .bold_line_style(WHITE.mix(0.3))
            .y_desc("ワイプ数")
            .x_desc("フェーズ")
            .axis_desc_style(("meiryo", 15))
            .draw()?;

        chart.draw_series(
            Histogram::vertical(&chart)
                .style(RED.mix(0.5).filled())
                .data(data.iter().map(|x| (SegmentValue::from(*x as u32), 1))),
        )?;

        // To avoid the IO failure being ignored silently, we manually call the present function
        root.present().unwrap();

        Ok(FILE_NAME.to_string())
    }
}
