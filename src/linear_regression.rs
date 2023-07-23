use plotters::backend::BitMapBackend;
use plotters::chart::ChartBuilder;
use plotters::style::full_palette::{RED, BLUE, WHITE};
use plotters::drawing::IntoDrawingArea;
use plotters::element::{Circle, EmptyElement};
use plotters::series::{LineSeries, PointSeries};


// Define a simple DataPoint struct
pub(crate) struct DataPoint {
    pub x: f64,
    pub y: f64,
}
struct LinearRegression{
    coefficient: f64,
    intercept: f64,
    goodness_of_fit:f64,
    residuals:f64,
    p_values_and_significance:f64,
    confidence_intervals:f64,
    predictions:f64,
    multicollinearity:f64,
    outliers:f64,
    homoscedasticity:f64,
}
// Function to calculate the linear regression coefficients
pub(crate) fn get_slope_intercept(data: &[DataPoint]) -> (f64, f64) {
    let n = data.len() as f64;
    let x_sum: f64 = data.iter().map(|p| p.x).sum();
    let y_sum: f64 = data.iter().map(|p| p.y).sum();
    let xy_sum: f64 = data.iter().map(|p| p.x * p.y).sum();
    let x_squared_sum: f64 = data.iter().map(|p| p.x * p.x).sum();

    let slope = (n * xy_sum - x_sum * y_sum) / (n * x_squared_sum - x_sum * x_sum);
    let intercept = (y_sum - slope * x_sum) / n;

    (slope, intercept)
}


// Function to plot the data and the linear regression line

pub(crate) fn plot_data_with_regression(
    data: &[DataPoint],
    slope: f64,
    intercept: f64,
    plot_title: &str,
    x_label: &str,
    y_label: &str,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a plotter area
    let output_file = &format!("./images/plots/{}",output_file);
    let root = BitMapBackend::new(output_file, (1600, 1200)).into_drawing_area();
    root.fill(&WHITE)?;

    // Define the plot area
    let mut chart = ChartBuilder::on(&root)
        .caption(plot_title, ("sans-serif", 20))
        .margin(5)
        .set_all_label_area_size(40)
        .build_cartesian_2d(0.0..6.0, 0.0..6.0)?;

    // Draw the scatter plot
    chart.configure_mesh().draw()?;
    chart.draw_series(PointSeries::of_element(
        data.iter().map(|p| (p.x, p.y)),
        5,
        &BLUE,
        &|c, s, st| {
            return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled());
        },
    ))?;

    // Draw the linear regression line
    chart.draw_series(LineSeries::new(
        vec![(0.0, intercept), (5.0, slope * 5.0 + intercept)],
        &RED,
    ))?;

    Ok(())
}


