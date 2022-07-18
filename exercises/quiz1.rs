fn calculate_apple_price(amount: u32) -> u32 {
    if amount > 40 {
        amount
    } else {
        amount * 2
    }
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(65, price3);
}
