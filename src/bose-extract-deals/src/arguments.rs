use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the SQLite database
    #[arg(short, long, default_value = "")]
    pub db_path: String,

    /// URL to fetch the HTML content from
    #[arg(
        short,
        long,
        default_value = "https://www.liverpool.com.mx/tienda/pdp/aud%C3%ADfonos-over-ear-bose-quietcomfort-ultra-se-sandstone-inal%C3%A1mbricos-con-cancelaci%C3%B3n-de-ruido/1150870956"
    )]
    pub url: String,
}

impl Args {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}
