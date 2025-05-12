fn main() {
    // Ingredient counts: [raspberry, eggs, sugar, milk_buckets, wheat_flour]
    let ingredients = vec![12, 15, 10, 8, 20];
    let desired_cakes = 11;

    raspberry_cake(&ingredients, desired_cakes);
}

fn raspberry_cake(ingredients: &[i32], desired_cakes: i32) {
    // Each cake needs 1 of each ingredient
    let cakes_possible = *ingredients.iter().min().unwrap_or(&0);
    if cakes_possible >= desired_cakes {
        println!("You have enough ingredients to make {desired_cakes} raspberry cakes!");
    } else {
        println!("You can only make {cakes_possible} raspberry cakes, not {desired_cakes}.");
    }
}