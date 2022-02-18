extern crate proc_macro;

use proc_macro::TokenStream;

fn main() {
    println!("Hello, world!");
}

/*
Should have been a macro that generates testcases, but does not work yet
#[macro_export]
macro_rules! multiply {
    let concatenated = format!("multitest__{}", $x);
    let varname = syn::Ident::new(&concatenated, $x.span());
    ( $( $x:expr ),* ) =>{
        $(
    #[test]
        quote! { fn #varname() {
        }
        }
        )*
    };
}

multiply![1,2,3,4];
*/

#[cfg(test)]
mod tests {

    #[test]
    fn atest() {}

    #[test]
    fn atest1() {}
    #[test]
    fn atest2() {}
    #[test]
    fn atest3() {}
    #[test]
    fn atest4() {}
    #[test]
    fn atest5() {}
    #[test]
    fn atest6() {}
    #[test]
    fn atest7() {}
}
