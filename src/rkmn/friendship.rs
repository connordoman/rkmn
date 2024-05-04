/*// Constants for AdjustFriendship
#define FRIENDSHIP_EVENT_GROW_LEVEL       0
#define FRIENDSHIP_EVENT_VITAMIN          1 // unused, handled by PokemonUseItemEffects
#define FRIENDSHIP_EVENT_BATTLE_ITEM      2 // unused, handled by PokemonUseItemEffects
#define FRIENDSHIP_EVENT_LEAGUE_BATTLE    3
#define FRIENDSHIP_EVENT_LEARN_TMHM       4
#define FRIENDSHIP_EVENT_WALKING          5
#define FRIENDSHIP_EVENT_FAINT_SMALL      6
#define FRIENDSHIP_EVENT_FAINT_FIELD_PSN  7
#define FRIENDSHIP_EVENT_FAINT_LARGE      8 // If opponent was >= 30 levels higher. See AdjustFriendshipOnBattleFaint

// Constants for GetLeadMonFriendshipScore
#define FRIENDSHIP_NONE        0
#define FRIENDSHIP_1_TO_49     1
#define FRIENDSHIP_50_TO_99    2
#define FRIENDSHIP_100_TO_149  3
#define FRIENDSHIP_150_TO_199  4
#define FRIENDSHIP_200_TO_254  5
#define FRIENDSHIP_MAX         6

// Friendship value that the majority of species use. This was changed in Generation 8 to 50.
#define STANDARD_FRIENDSHIP 70 */

pub trait Friendship {
    fn get_value(&self) -> u8;
}

pub enum FriendshipAdjustment {
    EventGrowLevel,
    EventVitamin,
    EventBattleItem,
    EventLeagueBattle,
    EventLearnTMHM,
    EventWalking,
    EventFaintSmall,
    EventFaintFieldPSN,
    EventFaintLarge,
}

pub enum FriendshipScore {
    None,
    Standard,
    OneToFortyNine,
    FiftyToNinetyNine,
    OneHundredToOneFortyNine,
    OneHundredFiftyToOneNinetyNine,
    TwoHundredToTwoFiftyFour,
    Max,
}

const STANDARD_FRIENDSHIP: u8 = 70;

impl Friendship for FriendshipScore {
    fn get_value(&self) -> u8 {
        match self {
            FriendshipScore::None => 0,
            FriendshipScore::Standard => STANDARD_FRIENDSHIP,
            FriendshipScore::OneToFortyNine => 50,
            FriendshipScore::FiftyToNinetyNine => 100,
            FriendshipScore::OneHundredToOneFortyNine => 150,
            FriendshipScore::OneHundredFiftyToOneNinetyNine => 200,
            FriendshipScore::TwoHundredToTwoFiftyFour => 255,
            FriendshipScore::Max => 255,
        }
    }
}
