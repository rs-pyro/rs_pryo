use plotters::prelude::*;

// Define a simple DataPoint struct
struct DataPoint {
    x: f64,
    y: f64,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example dataset: (x, y) pairs for the scatter plot
    let data = vec![
        DataPoint { x: 1.0, y: 2.0 },
        DataPoint { x: 2.0, y: 4.0 },
        DataPoint { x: 3.0, y: 5.0 },
        DataPoint { x: 4.0, y: 4.0 },
        DataPoint { x: 5.0, y: 5.0 },
    ];

    // Calculate the linear regression coefficients
    let (slope, intercept) = linear_regression(&data);

    // Print the results
    println!("Slope (Coefficient): {}", slope);
    println!("Intercept: {}", intercept);

    // Create a plotter area
    let root = BitMapBackend::new("scatter_plot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // Define the plot area
    let mut chart = ChartBuilder::on(&root)
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
// Linear regression function
fn linear_regression(data: &[DataPoint]) -> (f64, f64) {
    let n = data.len() as f64;
    let x_sum: f64 = data.iter().map(|p| p.x).sum();
    let y_sum: f64 = data.iter().map(|p| p.y).sum();
    let xy_sum: f64 = data.iter().map(|p| p.x * p.y).sum();
    let x_squared_sum: f64 = data.iter().map(|p| p.x * p.x).sum();

    let slope = (n * xy_sum - x_sum * y_sum) / (n * x_squared_sum - x_sum * x_sum);
    let intercept = (y_sum - slope * x_sum) / n;

    (slope, intercept)
}

fn mean(values: &Vec<f64>) -> f64 {
    let sum: f64 = values.iter().sum();
    sum / (values.len() as f64)
}

fn calc_covariance(x: &Vec<f64>, y: &Vec<f64>, x_mean: f64, y_mean: f64) -> f64 {
    let mut covariance = 0.0;
    for i in 0..x.len() {
        covariance += (x[i] - x_mean) * (y[i] - y_mean);
    }
    covariance / (x.len() as f64 - 1.0)
}

fn calc_variance(values: &Vec<f64>, mean: f64) -> f64 {
    let mut variance = 0.0;
    for &value in values {
        variance += (value - mean).powi(2);
    }
    variance / (values.len() as f64 - 1.0)
}