# rasciigraph
Tiny Rust library to draw pretty line graphs using ascii characters.

# Usage
Add this to your Cargo.toml
``` toml
[dependencies]
rasciigraph = "0.2"
```
Add this line of code to top of your source code
``` rust
extern crate rasciigraph;
```


# Examples
This code 
``` rust
use rasciigraph::consts::colors::RED;
use rasciigraph::plot::Config;
use rasciigraph::plot::Series;

fn main() {
    let cfg = Config::default();
    let res = rasciigraph::plot::plot(vec![
        Series::new(vec![8.0, 1.0, 10.0, 5.0, 4.0, 6.0], cfg.clone().color(RED)),
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

```
Produces an output like this
```
10.00┼ ╭╮                         
9.00 ┼ ││                         
8.00 ┼╮││                         
7.00 ┼││╭╮     ╭╮     ╭╮     ╭╮   
6.00 ┼│╭╯╰╮   ╭╯╰╮   ╭╯╰╮   ╭╯╰╮  
5.00 ┼╭╯╰╮╰╮ ╭╯  ╰╮ ╭╯  ╰╮ ╭╯  ╰╮ 
4.00 ┼╯│ ╰╯╰─╯    ╰─╯    ╰─╯    ╰ 
3.00 ┼││                          
2.00 ┼││                          
1.00 ┼╰╯                          
```

# Acknowledgement
This crate is rustlang port of library [asciigraph](https://github.com/guptarohit/asciigraph) written by [@guptarohit](https://github.com/guptarohit).

Above library is also port of library [asciichart](https://github.com/kroitor/asciichart) written by [@kroitor](https://github.com/kroitor).
