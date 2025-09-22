// Shared Bit Counter
#[allow(unused_variables)]
#[allow(non_snake_case)]
fn main() 
{
    println!("Shared Bit Counter");
    
    let num1: u8 = 1;
    // println!("Num1, Binary is {}", decimal_to_BCD(15));
    let num2: u8 = 2;

    let st1 = decimal_to_BCD(num1);
    let st2 = decimal_to_BCD(num2);
    let Val: bool = shared_bit_counter_strings_matching(st1, st2);
    println!("{}", Val);
    
    

}

#[allow(unused_assignments)]
#[allow(non_snake_case)]
fn decimal_to_BCD(dec: u8) -> String
{

    if dec == 0
    {
        return "0000".to_string();
    }
    else
    {
    
        if dec <=15
        {
            let mut quotient: u8  = dec;
            let mut dividend: u8  = dec;
            let mut remainder: u8 = 0;
            
            let mut BCD = Vec::new();
            let mut str_BCD = String::new();
            
            while quotient != 1
            {
                quotient  = dividend / 2;
                remainder = dividend % 2; 
                dividend  = quotient;
                
                //let mut remainder_string = format!("{}", remainder);
                BCD.push(remainder);
            }
            
            BCD.push(1);
            BCD.reverse();
            
            match BCD.len()
            {
                
                3 =>
                {
                    BCD.insert(0, 0);  
                }
                2 =>
                {
                    BCD.insert(0, 0); 
                    BCD.insert(0, 0); 
                }
                1 =>
                {
                    BCD.insert(0, 0); 
                    BCD.insert(0, 0); 
                    BCD.insert(0, 0); 
                }
                0 =>
                {
                    BCD.insert(0, 0); 
                    BCD.insert(0, 0); 
                    BCD.insert(0, 0); 
                    BCD.insert(0, 0); 
                }
                _ =>(),
            }
            
            
            
            for i in BCD.iter()
            {
                str_BCD.push_str(&i.to_string());
            }
            
            return str_BCD; 
        }
        else
        {
            panic!("Number is greater than 15, Please enter a number less than or equal to 15");
        }
    
    }
    

}


#[allow(unused_assignments)]
#[allow(non_snake_case)]
fn shared_bit_counter_strings_matching (string1: String, string2: String) -> bool
{
    if string1.is_empty() || string2.is_empty()
    {
        panic!("Strings are empty");
    }
    else
    {
        let mut match_indices: u8 = 0;
        

        for (index,ch) in string1.char_indices()
        {
            
            if ch == string2.chars().nth(index).expect("REASON")
            {
                if ch == '1'
                {
                    match_indices += 1;
                }
                
            }
        }

        if match_indices > 1
        {
            return true;
        }
        else
        {
            return false;
        }
    }
}
