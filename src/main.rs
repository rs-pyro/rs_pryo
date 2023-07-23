mod statistics;
mod linear_regression;

use linear_regression::get_slope_intercept;
use linear_regression::plot_data_with_regression;
use linear_regression::DataPoint;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example dataset: (x, y) pairs for the scatter plot
    let data = vec![
        DataPoint { x: 1.0, y: 2.0 },
        DataPoint { x: 2.0, y: 4.0 },
        DataPoint { x: 3.0, y: 5.0 },
        DataPoint { x: 4.0, y: 4.0 },
        DataPoint { x: 5.0, y: 5.0 },
        DataPoint { x: 5.0, y: 4.0 },
        DataPoint { x: 1.5, y: 2.5 },
        DataPoint { x: 3.2, y: 4.8 },
        DataPoint { x: 4.8, y: 5.3 },
    ];

    // Calculate the linear regression coefficients
    let (slope, intercept) = get_slope_intercept(&data);

    // Print the results
    println!("Slope (coefficient): {}", slope);
    println!("intercept: {}", intercept);

    // Plot the data with the linear regression line
    plot_data_with_regression(
        &data,
        slope,
        intercept,
        "Scatter Plot with Linear Regression",
        "X Axis",
        "Y Axis",
        "scatter_plot.png",
    )?;

    Ok(())
}