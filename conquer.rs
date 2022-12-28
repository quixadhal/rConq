#![allow(dead_code)]

use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::SeqCst;
use std::fmt;

#[derive(Debug)]
#[derive(Clone)]
enum Elevation {
	WATER,
	PEAK,
	MOUNTAIN,
	HILL,
	CLEAR,
	UNKNOWN,
}

impl Elevation {
    // To get the integer version, just cast it "as i32".
    fn to_char(&self) -> char {
        match self {
            Elevation::WATER       => '~',
            Elevation::PEAK        => '#',
            Elevation::MOUNTAIN    => '^',
            Elevation::HILL        => '%',
            Elevation::CLEAR       => '-',
            _                      => '?',
        }
    }
    fn to_string(&self) -> String {
        match self {
            Elevation::WATER        => "WATER".to_string(),
            Elevation::PEAK         => "PEAK".to_string(),
            Elevation::MOUNTAIN     => "MOUNTAIN".to_string(),
            Elevation::HILL         => "HILL".to_string(),
            Elevation::CLEAR        => "CLEAR".to_string(),
            _                       => "UNKNOWN".to_string(),
        }
    }
}

impl fmt::Display for Elevation {
    // error[E0277]: `Elevation` doesn't implement `std::fmt::Display`
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<char> for Elevation {
    fn from(c: char) -> Self {
        match c {
            '~' => Elevation::WATER,
            '#' => Elevation::PEAK,
            '^' => Elevation::MOUNTAIN,
            '%' => Elevation::HILL,
            '-' => Elevation::CLEAR,
            _   => Elevation::UNKNOWN,
        }
    }
}

impl From<Elevation> for char {
    fn from(e: Elevation) -> Self {
        e.to_char()
    }
}

impl From<Elevation> for String {
    fn from(e: Elevation) -> Self {
        e.to_string()
    }
}

impl From<Elevation> for i32 {
    fn from(e: Elevation) -> Self {
        e as i32
    }
}

#[derive(Debug)]
#[derive(Clone)]
enum Race {
    GOD,
    ORC,
    ELF,
    DWARF,
    LIZARD,
    HUMAN,
    PIRATE,
    SAVAGE,
    NOMAD,
    UNKNOWN,
}

impl Race {
    // To get the integer version, just cast it "as i32".
    fn to_char(&self) -> char {
        match self {
            Race::GOD           => '-',
            Race::ORC           => 'O',
            Race::ELF           => 'E',
            Race::DWARF         => 'D',
            Race::LIZARD        => 'L',
            Race::HUMAN         => 'H',
            Race::PIRATE        => 'P',
            Race::SAVAGE        => 'S',
            Race::NOMAD         => 'N',
            _                   => '?',
        }
    }

    fn to_string(&self) -> String {
        match self {
            Race::GOD           => "GOD".to_string(),
            Race::ORC           => "ORC".to_string(),
            Race::ELF           => "ELF".to_string(),
            Race::DWARF         => "DWARF".to_string(),
            Race::LIZARD        => "LIZARD".to_string(),
            Race::HUMAN         => "HUMAN".to_string(),
            Race::PIRATE        => "PIRATE".to_string(),
            Race::SAVAGE        => "SAVAGE".to_string(),
            Race::NOMAD         => "NOMAD".to_string(),
            _                   => "UNKNOWN".to_string(),
        }
    }
}

impl fmt::Display for Race {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<char> for Race {
    fn from(c: char) -> Self {
        match c {
            '-' => Race::GOD,
            'O' => Race::ORC,
            'E' => Race::ELF,
            'D' => Race::DWARF,
            'L' => Race::LIZARD,
            'H' => Race::HUMAN,
            'P' => Race::PIRATE,
            'S' => Race::SAVAGE,
            'N' => Race::NOMAD,
            _   => Race::UNKNOWN,
        }
    }
}

impl From<Race> for char {
    fn from(r: Race) -> Self {
        r.to_char()
    }
}

impl From<Race> for String {
    fn from(r: Race) -> Self {
        r.to_string()
    }
}

impl From<Race> for i32 {
    fn from(r: Race) -> Self {
        r as i32
    }
}


#[derive(Debug)]
#[derive(Clone)]
struct Army {
    unittyp         : u8,
    xloc            : u8,
    yloc            : u8,
    smove           : u8,
    sold            : i32,
    stat            : u8,
}

impl Default for Army {
    fn default() -> Army {
        Army {
            unittyp    : 0,
            xloc       : 0,
            yloc       : 0,
            smove      : 0,
            sold       : 0,
            stat       : 0,
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
struct Navy {
    warships        : u16,
    merchant        : u16,
    galleys         : u16,
    xloc            : u8,
    yloc            : u8,
    smove           : u8,               // movement ability of ship
    crew            : u8,               // crew on ship
    people          : u8,               // people carried
    commodity       : u8,               // future commodities
    armynum         : u8,               // army carried
}

impl Default for Navy {
    fn default() -> Navy {
        Navy {
            warships        : 0,
            merchant        : 0,
            galleys         : 0,
            xloc            : 0,
            yloc            : 0,
            smove           : 0,
            crew            : 0,
            people          : 0,
            commodity       : 0,
            armynum         : 0,
        }
    }
}

#[derive(Debug)]
//#[derive(Copy)]
#[derive(Clone)]
struct Nation {
    id              : i32,          // unique naiton ID, established at allocation

    name            : String,       // name, we do not want to deal with char arrays
    passwd          : String,       // password
    leader          : String,       // leader title

    race            : Race,         // national race
    location        : i8,           // starting location quality, good, fair, random
    mark            : char,         // unique symbol for nation display on map

    capx            : u8,           // capitol x coordiante, is 16 bit in world
    capy            : u8,           // capitol y coordinate, is 16 bit in world
    active          : u8,           // nation type and strategy
    maxmove         : u8,           // maximum movement of soldiers
    repro           : i8,           // reproduction rate of nation

    score           : i32,          // score
    tgold           : i32,          // gold in treasury
    jewels          : i32,          // raw amount of jewels in treasury
    tmil            : i32,          // total military
    tciv            : i32,          // total civilians
    metals          : i32,          // total real metal in nation
    tfood           : i32,          // total food in nation
    powers          : i32,          // bit vector, should become boolean array

    class           : i16,          // national class
    aplus           : i16,          // attack plus of all soldiers
    dplus           : i16,          // attack plus of all soldiers
    spellpts        : i16,          // spell points
    tsctrs          : i16,          // total number sectors
    tships          : i16,          // number warships
    inflation       : i16,          // inflation rate

    charity         : u8,           // charity budget (% of Taxes)

    arm             : Vec<Army>,    // array of army units
    nvy             : Vec<Navy>,    // array of naval units
    dstatus         : Vec<i8>,      // diplomatic status array

    tax_rate        : u8,           // taxrate populace
    prestige        : u8,           // nations prestige
    popularity      : u8,           // governments popularity
    power           : u8,           // nation power
    communications  : u8,           // leader communication
    wealth          : u8,           // per capita income
    eatrate         : u8,           // food eaten / 10 people
    spoilrate       : u8,           // food spoilage rate
    knowledge       : u8,           // general knowledge
    farm_ability    : u8,           // farming ability
    mine_ability    : u8,           // mine ability
    poverty         : u8,           // % poor people
    terror          : u8,           // peoples terror of you
    reputation      : u8,           // reputation of nation
}

impl Nation {
    fn unique_id() -> i32 {
        static COUNTER : AtomicI32 = AtomicI32::new(0);

        let id = COUNTER.fetch_add(1, SeqCst);
        assert_ne!(id, i32::MAX, "ID counter has overflowed");
        id
    }
}

impl Default for Nation {
    fn default() -> Nation {
        Nation {
            id              : Nation::unique_id(),

            name            : String::from("Bob"),
            passwd          : String::from(""),
            leader          : String::from(""),

            race            : Race::UNKNOWN,
            location        : 0,
            mark            : ' ',

            capx            : 0,
            capy            : 0,
            active          : 0,
            maxmove         : 0,
            repro           : 0,

            score           : 0,
            tgold           : 0,
            jewels          : 0,
            tmil            : 0,
            tciv            : 0,
            metals          : 0,
            tfood           : 0,
            powers          : 0,

            class           : 0,
            aplus           : 0,
            dplus           : 0,
            spellpts        : 0,
            tsctrs          : 0,
            tships          : 0,
            inflation       : 0,

            charity         : 0,

            arm             : Vec::new(),
            nvy             : Vec::new(),
            dstatus         : Vec::new(),

            tax_rate        : 0,
            prestige        : 0,
            popularity      : 0,
            power           : 0,
            communications  : 0,
            wealth          : 0,
            eatrate         : 0,
            spoilrate       : 0,
            knowledge       : 0,
            farm_ability    : 0,
            mine_ability    : 0,
            poverty         : 0,
            terror          : 0,
            reputation      : 0,
        }
    }
}

#[derive(Debug)]
struct Sector {
    designation         : u8,       // designation of sector
    altitude            : Elevation,       // sector altitude
    vegetation          : u8,       // sector vegetation
    owner               : u8,       // nation id of owner
    people              : i32,      // civilians in sector
    i_people            : i16,      // initial civilians in sector
    jewels              : u8,       // jewel production ability
    fortress            : u8,       // fortification level; 0 to 12
    metal               : u8,       // metal produced by sector
    tradegood           : u8,       // exotic trade goods in sector
    region              : u8,       // unused, index of region
}

impl Default for Sector {
    fn default() -> Sector {
        Sector {
            designation     : 0,
            altitude        : Elevation::WATER,
            vegetation      : 0,
            owner           : 0,
            people          : 0,
            i_people        : 0,
            jewels          : 0,
            fortress        : 0, 
            metal           : 0,
            tradegood       : 0,
            region          : 0,
        }
    }
}

#[derive(Debug)]
struct World {
    mapx        : i16,          // size of world
    mapy        : i16,          // size of world
    othrntns    : i16,          // god, lizard...
    turn        : i16,          // game turn
    m_mil       : i32,          // number of mercs available
    m_aplus     : i16,          // mercenary attack bonus
    m_dplus     : i16,          // mercenary defense bonus
    w_jewels    : i32,          // jewels in world
    w_gold      : i32,          // gold talons in world
    w_food      : i32,          // food in world
    w_metal     : i32,          // metal in world
    w_civ       : i32,          // world population
    w_mil       : i32,          // world military
    w_sctrs     : i32,          // owned sectors in world
    score       : i32,          // world score total
    ntn         : Vec<Nation>,  // not present in original
}

impl Default for World {
    fn default() -> World {
        World {
            mapx        : 0,
            mapy        : 0,
            othrntns    : 0,
            turn        : 0,
            m_mil       : 0,
            m_aplus     : 0,
            m_dplus     : 0,
            w_jewels    : 0,
            w_gold      : 0,
            w_food      : 0,
            w_metal     : 0,
            w_civ       : 0,
            w_mil       : 0,
            w_sctrs     : 0,
            score       : 0,
            ntn         : Vec::new(),
        }
    }
}

use Elevation::*;

fn main() {
    println!("This is just a test.");
    let mut w : World = Default::default();
    println!("{} nations exist.", w.ntn.len());
    println!("w {:?}", w);

    let mut n : Nation = Default::default();
    println!("n {:?}", n);
    w.ntn.push(n.clone());
    println!("----");
    println!("{} nations exist.", w.ntn.len());
    println!("w {:?}", w);

    n.name = String::from("Ted");
    println!("----");
    println!("w {:?}", w);

    n = Default::default(); // hmm
    n.name = String::from("George");
    w.ntn.push(n.clone());
    println!("----");
    println!("{} nations exist.", w.ntn.len());
    println!("n {:?}", n);
    println!("w {:?}", w);

    n = Default::default(); // hmm
    n.name = String::from("Fred");
    w.ntn.push(n.clone());
    println!("----");
    println!("{} nations exist.", w.ntn.len());

    for i in &w.ntn {
        println!("Nation ID {} for {}", i.id, i.name);
    }

    println!("Murdering {}!", w.ntn[1].name);
    w.ntn.remove(1);
    println!("{} nations exist.", w.ntn.len());

    for i in &w.ntn {
        println!("Nation ID {} for {}", i.id, i.name);
    }

    println!("Just checking MOUNTAIN:");
    println!("    to_string() {},", MOUNTAIN.to_string());
    println!("    to_char() {},", MOUNTAIN.to_char());
    println!("    as i32 {},", MOUNTAIN as i32);
    println!("    char::from {},", char::from(MOUNTAIN));
    println!("    String::from {},", String::from(MOUNTAIN));
    println!("    i32::from {},", i32::from(MOUNTAIN));
    println!("    {}", MOUNTAIN);

    let x : Elevation = '^'.into();
    println!("    x {}", x);

    println!("Test completed.");

}
