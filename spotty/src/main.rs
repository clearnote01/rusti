mod spotinf;

use spotinf::cur_song;
 
fn main() {
    let song = cur_song();
    println!("Current song is : {}", song.title);
    println!("Current artist is : {}", song.artist);
    println!("Current song status is : {:?}", song.status);
}
