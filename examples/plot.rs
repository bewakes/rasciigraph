use rasciigraph::consts::colors::RED;
use rasciigraph::plot::Config;
use rasciigraph::plot::Series;

fn main() {
    let cfg = Config::default();
    let res = rasciigraph::plot::plot(vec![
        Series::new(vec![20.0, 1.0, 10.0, 5.0, 4.0, 6.0], cfg.clone().color(RED)),
        Series::new(
            vec![
                4.0, 5.0, 6.0, 7.0, 6.0, 5.0, 4.0, 4.0, 5.0, 6.0, 7.0, 6.0, 5.0, 4.0, 4.0, 5.0,
                6.0, 7.0, 6.0, 5.0, 4.0, 4.0, 5.0, 6.0, 7.0, 6.0, 5.0, 4.0,
            ],
            cfg.clone().color("32"),
        ),
    ]);
    println!("{}", res);
}
