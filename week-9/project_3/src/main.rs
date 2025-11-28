
fn main() {

    let comm_arr:[&str;6] = ["ii","| Aigbogun Alamba Daudu","| Murtala Afeez Bundu","| Okorocha Calistud Ogbona","| Adewale Jimoh Akanbi","| Osazuwa Faith Etieye"];
    let min_arr:[&str;6] = ["AA","   | Internal Affairs","     | Justice","| Defence","    | Power & Steel","    | Petroleum"];
    let geo_arr:[&str;6] = ["UU"," | South West","          | North East","          | South South","    | South West","        | South East"];
    println!("-------------------------------------------------------------------------");
    println!("S/N | NAME OF COMMISIONER       | MINISTRY           | GEOPOLITICAL ZONE"); 
    println!("-------------------------------------------------------------------------");
    for index in 1..6
    { println!("{}   {}  {}  {}", index, comm_arr[index],min_arr[index],geo_arr[index]); }

}