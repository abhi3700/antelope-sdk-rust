//! API Endpoints for different Antelope networks: https://eos.antelope.tools/endpoints

// get chain info
mod get_info;

fn main() {
    let _ = get_info::get_block();
}
