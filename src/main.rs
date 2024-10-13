fn main() {
    let triangles = 6; // задайте нужное количество треугольников
    draw_tree(triangles);
}

fn draw_tree(triangles: usize) {
    let width = triangles * 2 + 1;

    (1..=triangles).for_each(|triangle| {
        (0..triangle).for_each(|row| {
            let stars = 2 * row + 1;
            let spaces = (width - stars) / 2;

            println!(
                "{}{}",
                " ".repeat(spaces),
                "*".repeat(stars)
            );
        });
    });
}
