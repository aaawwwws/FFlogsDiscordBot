use std::collections::HashMap;

use plotters::prelude::*;

pub struct WipeGraph;
impl WipeGraph {
    pub fn new() -> Self {
        return Self;
    }

    pub fn create_graph(
        &self,
        data: &Vec<u8>,
        area_name: &str,
    ) -> anyhow::Result<(&str, u32, u8)> {
        const FILE_NAME: &str = "graph.png";
        if data.is_empty() {
            return Err(anyhow::anyhow!("配列が見つかりません。"));
        }

        let mut map: HashMap<u8, u8> = HashMap::new();

        for d in data {
            if let Some(a) = map.get(d) {
                map.insert(*d, *a + 1);
            }
            map.insert(*d, 1);
        }

        let max_wipe: u32 = *map.values().max().unwrap() as u32;

        let (max_phase, _) = map.into_iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();

        let root = BitMapBackend::new(FILE_NAME, (640, 480)).into_drawing_area();

        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(35)
            .y_label_area_size(40)
            .margin(5)
            .caption(area_name, ("meiryo", 40))
            .build_cartesian_2d(
                (1u32..*data.iter().max().unwrap() as u32).into_segmented(),
                0u32..max_wipe + 1,
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
        Ok((FILE_NAME, max_wipe, max_phase))
    }
}
