
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash::hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::*;
use smash::phx::Hash40;



/*
Slots:
C00 = Steve = 0
C01 = Alex  = 1
C02 = Java  = 2
C03 = Bedrock = 3
C04 = Skeleton = 4
C05 = Zombie Piglin = 5
C06 = Zombie = 6
C07 = Enderman = 7
*/

/*Note C00 and C01 will not be included as they are fine in-game.
Both Steve and Alex are Minecraft Representatives
C02 and C03 are a mix of Minecraft and Minecraft dungeons
C04, C05, C06, C07 all represent Minecraft and Minecraft dungeons. C05 is more of the old Minecraft. And EVERYONE is in this Script
*/

//Status Script Death sounds
/* 	0 = Blast zone or hero blasing enderman out of existance
	1 = Star KO
	2 = Screen KO, Yes this is also giga punch.
	3 = Star KO if galaga grabs you
	Stamina is a situation
*/




////////////////////////////////////////////////////////////////////Sounds\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\




#[acmd_script( agent = "pickel", scripts = ["sound_appealhir", "sound_appealhil"] , category = ACMD_SOUND, low_priority )]
unsafe fn appealhi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4 {
        acmd!(lua_state,{
            frame(Frame=44)
            if(is_excute){
                PLAY_SE(hash40("se_pickel_appealhi"))
			}
			frame(Frame=72)
            if(is_excute){
                PLAY_SE(hash40("se_pickel_landing_high_place"))
            }
        });
    }
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
        acmd!(lua_state,{
            frame(Frame=3)
            if(is_excute){
                PLAY_SE(hash40("se_pickel_appealhi"))
            }
        });
    }
    else {
        acmd!(lua_state, {
			if(is_excute){
				PLAY_SE(hash40("se_pickel_jump01"))
			}
			frame(Frame=17)
			if(is_excute){
				PLAY_SE(hash40("se_pickel_swing_s"))
			}
			frame(Frame=34)
			if(is_excute){
				PLAY_SE(hash40("se_pickel_swing_s"))
			}
			frame(Frame=35)
			if(is_excute){
				PLAY_STEP(hash40("se_pickel_landing01"))
			}
			frame(Frame=57)
			if(is_excute){
				PLAY_SE(hash40("se_pickel_swing_s"))
			}
			frame(Frame=70)
			if(is_excute){
				PLAY_STEP(hash40("se_pickel_landing01"))
			}
		});
	}
}


#[acmd_script( agent = "pickel", scripts = ["sound_entryl", "sound_entryr"] , category = ACMD_SOUND, low_priority )]
unsafe fn entry_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4{
        acmd!(lua_state,{
            frame(Frame=2)
            if(is_excute){
                PLAY_SE(hash40("se_pickel_left_stone_02_m"))
	    }
	    frame(Frame=25)
	    if(is_excute){
                PLAY_SE(hash40("se_pickel_left_stone_03_m"))
	    }
	    frame(Frame=50)
	    if(is_excute){
                PLAY_SE(hash40("se_pickel_right_stone_04_m"))
	    }
	    frame(Frame=75)
	    if(is_excute){
                PLAY_SE(hash40("se_pickel_step_stone01_m"))
	    }
	    frame(Frame=30)
	    if(is_excute){
                PLAY_SE(hash40("se_pickel_left_stone_01_m"))
            }
        });
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6
	||
	WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7{
        acmd!(lua_state,{
            frame(Frame=3)
            if(is_excute){
                PLAY_SE(hash40("se_pickel_entry"))
            }
        });
    }
    else {
	acmd!(lua_state, {
            frame(Frame=3)
            if(is_excute){
                PLAY_SE(hash40("se_pickel_special_n01_rock"))
            }
            frame(Frame=13)
            if(is_excute){
                PLAY_SE(hash40("se_pickel_special_n01_rock"))
            }
            frame(Frame=24)
            if(is_excute){
                PLAY_SE(hash40("se_pickel_special_n01_rock"))
            }
            frame(Frame=26)
            if(is_excute){
                PLAY_SE(hash40("se_pickel_special_n02_rock"))
            }
            frame(Frame=36)
            if(is_excute){
                PLAY_SE(hash40("se_pickel_special_n01_rock"))
            }
            frame(Frame=46)
            if(is_excute){
                PLAY_SE(hash40("se_pickel_special_n01_rock"))
            }
            frame(Frame=56)
            if(is_excute){
                PLAY_SE(hash40("se_pickel_special_n01_rock"))
            }
            frame(Frame=62)
            if(is_excute){
                PLAY_SE(hash40("se_pickel_special_n02_rock"))
            }
            frame(Frame=97)
            if(is_excute){
                PLAY_SE(hash40("se_pickel_step_stone04_ll"))
            }
            frame(Frame=112)
            if(is_excute){
                PLAY_SE(hash40("se_pickel_step_stone05_ll"))
            }
        });
    }
}


#[acmd_script( agent = "pickel", scripts = ["sound_damageflyhi", "sound_damageflylw", "sound_damageflyn", "sound_damageflyroll", "sound_damageflytop", 
"sound_downdamaged", "sound_downdamaged3", "sound_downdamageu", "sound_downdamageu3"] , category = ACMD_SOUND, low_priority )]
unsafe fn damage_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if  WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4
	    ||
	WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5
	    ||
	WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6
	    ||
	   WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
        acmd!(lua_state,{
            frame(Frame=1)
            if(is_excute){
                PLAY_SE(hash40("se_pickel_landing_high_place"))
            }
        });
    }
	else {	
    acmd!(lua_state, {
			frame(Frame=43)
			if(is_excute){
				//OG Script empty
			}
		});
	}
}

#[acmd_script( agent = "pickel", scripts = ["sound_appealsr", "sound_appealsl"] , category = ACMD_SOUND, low_priority )]
unsafe fn appeals_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4 {
        acmd!(lua_state,{
            frame(Frame=32)
            if(is_excute){
                PLAY_SE(hash40("se_pickel_appeals_s01_01"))
			}
			frame(Frame=67)
            if(is_excute){
                PLAY_SE(hash40("se_pickel_appeals_s01_02"))
			}
			frame(Frame=162)
            if(is_excute){
                PLAY_SE(hash40("se_pickel_appeals_s01_03"))
            }
        });
    }
    else {
        acmd!(lua_state, {
			frame(Frame=18)
			if(is_excute){
				PLAY_SE(hash40("se_pickel_appeal_s01"))
			}
			frame(Frame=30)
			if(is_excute){
				PLAY_SE(hash40("se_pickel_appeal_s01"))
			}
			frame(Frame=42)
			if(is_excute){
				PLAY_SE(hash40("se_pickel_appeal_s01"))
			}
			frame(Frame=54)
			if(is_excute){
				PLAY_SE(hash40("se_pickel_appeal_s01"))
			}
			frame(Frame=68)
			if(is_excute){
				PLAY_SE(hash40("se_pickel_appeal_s01"))
			}
			frame(Frame=80)
			if(is_excute){
				PLAY_SE(hash40("se_pickel_appeal_s01"))
			}
			frame(Frame=91)
			if(is_excute){
				PLAY_SE(hash40("se_pickel_appeal_s02"))
			}
		});
	}
}

#[acmd_script( agent = "pickel", scripts = ["sound_appeallwr", "sound_appeallwl"] , category = ACMD_SOUND, low_priority )]
unsafe fn appeallw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4
		||
	   WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7	{
        acmd!(lua_state,{
            frame(Frame=3)
            if(is_excute){
                PLAY_SE(hash40("se_pickel_appeallw"))
            }
        });
    }
	else {
        acmd!(lua_state, {
			frame(Frame=1)
			if(is_excute){

			}
		});
	}
}

#[acmd_script( agent = "pickel", scripts = ["sound_win2"] , category = ACMD_SOUND, low_priority )]
unsafe fn win2_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4 {
        acmd!(lua_state,{
            frame(Frame=30)
			if(is_excute){
			PLAY_SE(hash40("se_pickel_win02_01"))
			}
			frame(Frame=40)
			if(is_excute){
				
		    }
			frame(Frame=50)
			if(is_excute){
			PLAY_SE(hash40("se_pickel_win02_01"))
			}
			frame(Frame=60)
			if(is_excute){
			}
			frame(Frame=66)
			if(is_excute){
			PLAY_SE(hash40("se_pickel_win02_01"))
			}
			frame(Frame=76)
			if(is_excute){
				
			}
			frame(Frame=82)
			if(is_excute){
			PLAY_SE(hash40("se_pickel_win02_01"))
			}
			frame(Frame=92)
			if(is_excute){
			
			}
			frame(Frame=98)
			if(is_excute){
			PLAY_SE(hash40("se_pickel_win02_01"))
			
			}
			frame(Frame=108)
			if(is_excute){
			PLAY_SE(hash40("se_pickel_win02_01"))
            }
        });
    }
    else {
        acmd!(lua_state, {
		frame(Frame=46)
		if(is_excute){
			PLAY_SE(hash40("se_pickel_win02_01"))
			}
		});
	}
}









////////////////////////////////////////////////////////////////////Sounds\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\





///////////////////////////////////////////////////////////////////Effects\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\
#[acmd_script( agent = "pickel", scripts = ["effect_entryr", "effect_entryl"] , category = ACMD_EFFECT, low_priority )]
unsafe fn entry_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4 {
        acmd!(lua_state,{
            if(is_excute){
			frame(Frame=1)
            }
        });
    }
    else{
        acmd!(lua_state, {
			frame(Frame=30)
			if(is_excute){
				EFFECT(hash40("pickel_block_break_stone"), hash40("top"), -4.5, 13.5, -0.25, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
			}
			frame(Frame=63)
			if(is_excute){
				EFFECT(hash40("pickel_block_break_stone"), hash40("top"), -4.5, 4.5, -0.25, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)
			}
		});
	}
}



#[acmd_script( agent = "pickel", scripts = ["effect_appeallwr", "effect_appeallwl"] , category = ACMD_EFFECT, low_priority )]
unsafe fn appeallw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4 {
        acmd!(lua_state,{
            if(is_excute){
			frame(Frame=32)
			EFFECT_FOLLOW(hash40("pickel_elytra_smoke"), hash40("haver"), 0, 0, 0, 0, 0, 0, 1, true)
			EFFECT_FOLLOW(hash40("pickel_elytra_smoke"), hash40("havel"), 0, 0, 0, 0, 0, 0, 1, true)
			}
			if(is_excute){
			frame(Frame=50)
			EFFECT_OFF_KIND(hash40("pickel_elytra_smoke"), true, true)
            }
        });
    }
    else{
        acmd!(lua_state, {
			frame(Frame=2)
			if(is_excute){
				FOOT_EFFECT(hash40("null"), hash40("top"), -2, 0, -2.5, 0, -15, 0, 0.6, 0, 0, 0, 0, 0, 0, false)
			}
			frame(Frame=29)
			if(is_excute){
				FOOT_EFFECT(hash40("null"), hash40("top"), -2, 0, -2.5, 0, -15, 0, 0.5, 0, 0, 0, 0, 0, 0, false)
			}
			frame(Frame=42)
			if(is_excute){
				FOOT_EFFECT(hash40("null"), hash40("top"), -2, 0, -2.5, 0, -15, 0, 0.5, 0, 0, 0, 0, 0, 0, false)
			}
		});
	}
}

#[acmd_script( agent = "pickel", scripts = ["effect_appealhir", "effect_appealhil"] , category = ACMD_EFFECT, low_priority )]
unsafe fn appealhi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4
	    ||
	   WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7{
        acmd!(lua_state,{
            frame(Frame=35)
            if(is_excute){
				FOOT_EFFECT(hash40("sys_landing_smoke_s"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false)
            }
        });
    }
    else {
        acmd!(lua_state, {
			frame(Frame=35)
			if(is_excute){
				FOOT_EFFECT(hash40("sys_landing_smoke_s"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false)
			}
			frame(Frame=70)
			if(is_excute){
				LANDING_EFFECT(hash40("sys_down_smoke"), hash40("top"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false)
				LAST_EFFECT_SET_RATE(1.1)
			}
		});
	}
}

#[acmd_script( agent = "pickel", scripts = ["effect_appealsr", "effect_appealsl"] , category = ACMD_EFFECT, low_priority )]
unsafe fn appealsr_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4 {
        acmd!(lua_state,{
            if(is_excute){
			frame(Frame=33)
			EFFECT_FOLLOW(hash40("pickel_elytra_smoke"), hash40("haver"), 0, 0, 0, 0, 0, 0, 1, true)
			EFFECT_FOLLOW(hash40("pickel_elytra_smoke"), hash40("havel"), 0, 0, 0, 0, 0, 0, 1, true)
			}
			if(is_excute){
			frame(Frame=66)
			EFFECT_OFF_KIND(hash40("pickel_elytra_smoke"), true, true)
            }
        });
    }
    else {
        acmd!(lua_state, {
			frame(Frame=18)
			if(is_excute){
				EFFECT_FOLLOW(hash40("pickel_eat_meat"), hash40("head"), 0.5, 2.5, 0, 0, 0, 0, 0.8, true)
			}
			frame(Frame=79)
			if(is_excute){
				EFFECT_OFF_KIND(hash40("pickel_eat_meat"), false, true)
			}
		});
	}
}

#[acmd_script( agent = "pickel", scripts = ["effect_visualscene"] , category = ACMD_EFFECT, low_priority )]
unsafe fn visualscene_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4
		||
	   WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7{
        acmd!(lua_state,{
            if(is_excute){
				EFFECT(hash40("pickel_final_white2"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
			}
			frame(Frame=14)
			if(is_excute){
				EFFECT(hash40("sys_landing_smoke"), hash40("top"), -8, 57, -3, 0, 90, 0, 0.7, 0, 0, 0, 0, 0, 0, true)
			}
			frame(Frame=280)
			if(is_excute){
				EFFECT(hash40("pickel_final_white"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
			}
        });
    }
    else {
        acmd!(lua_state, {
			if(is_excute){
				EFFECT(hash40("pickel_final_white2"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
			}
			frame(Frame=14)
			if(is_excute){
				EFFECT(hash40("sys_landing_smoke"), hash40("top"), -8, 57, -3, 0, 90, 0, 0.7, 0, 0, 0, 0, 0, 0, true)
			}
			frame(Frame=250)
			if(is_excute){
				EFFECT_FOLLOW(hash40("pickel_eat_meat"), hash40("head"), 0.5, 2.5, 0, 0, 0, 0, 0.8, true)
			}
			frame(Frame=280)
			if(is_excute){
				EFFECT(hash40("pickel_final_white"), hash40("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
			}
			frame(Frame=311)
			if(is_excute){
				EFFECT_OFF_KIND(hash40("pickel_eat_meat"), false, true)
			}
		});
	}
}
#[acmd_script( agent = "pickel", scripts = ["effect_win2"] , category = ACMD_EFFECT, low_priority )]
unsafe fn win2_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4 {
        acmd!(lua_state,{
			frame(Frame=30)
			if(is_excute){
			EFFECT_FOLLOW(hash40("pickel_elytra_smoke"), hash40("haver"), 0, 0, 0, 0, 0, 0, 1, true)
			}
			frame(Frame=40)
			if(is_excute){
			EFFECT_OFF_KIND(hash40("pickel_elytra_smoke"), true, true)
		    }
			frame(Frame=50)
			if(is_excute){
			EFFECT_FOLLOW(hash40("pickel_elytra_smoke"), hash40("haver"), 0, 0, 0, 0, 0, 0, 1, true)
			}
			frame(Frame=60)
			if(is_excute){
			EFFECT_OFF_KIND(hash40("pickel_elytra_smoke"), true, true)
			}
			frame(Frame=66)
			if(is_excute){
			EFFECT_FOLLOW(hash40("pickel_elytra_smoke"), hash40("haver"), 0, 0, 0, 0, 0, 0, 1, true)
			}
			frame(Frame=76)
			if(is_excute){
			EFFECT_OFF_KIND(hash40("pickel_elytra_smoke"), true, true)	
			}
			frame(Frame=82)
			if(is_excute){
			EFFECT_FOLLOW(hash40("pickel_elytra_smoke"), hash40("haver"), 0, 0, 0, 0, 0, 0, 1, true)
			}
			frame(Frame=92)
			if(is_excute){
			EFFECT_OFF_KIND(hash40("pickel_elytra_smoke"), true, true)
			}
			frame(Frame=98)
			if(is_excute){
			EFFECT_FOLLOW(hash40("pickel_elytra_smoke"), hash40("haver"), 0, 0, 0, 0, 0, 0, 1, true)
			}
			frame(Frame=108)
			if(is_excute){
			EFFECT_OFF_KIND(hash40("pickel_elytra_smoke"), true, true)
            }
        });
    }
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
        acmd!(lua_state,{
            frame(Frame=3)
            if(is_excute){
              //No Effect
            }
        });
    }
    else {
        acmd!(lua_state, {
			frame(Frame=43)
			if(is_excute){
				EFFECT_FOLLOW(hash40("pickel_eat_meat"), hash40("head"), 0, 2.7, 0, 0, 0, 0, 0.8, true)
				LAST_EFFECT_SET_RATE(1.3)
			}
			frame(Frame=124)
			if(is_excute){
				EFFECT_OFF_KIND(hash40("pickel_eat_meat"), true, true)
			}
		});
	}
}

///////////////////////////////////////////////////////////////////Effects\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\



////////////////////////////////////////////////////////////////////Game\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\
#[acmd_script( agent = "pickel", scripts = ["game_appeallwr", "game_appeallwl"] , category = ACMD_GAME, low_priority )]
unsafe fn appeallw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4
	    ||
       WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6
	    ||
       WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
        acmd!(lua_state,{
            if(is_excute){
              WorkModule::on_flag(Flag=FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON)
            }
        });
    }	
    else {
        acmd!(lua_state, {
            frame(Frame=35)
			
			if(is_excute){

			}
		});
	}
}
#[acmd_script( agent = "pickel", scripts = ["game_entryr", "game_entryl"] , category = ACMD_GAME, low_priority )]
unsafe fn entry_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 4
	    ||
	   WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 5
	    ||
	   WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
        acmd!(lua_state,{
            if(is_excute){
              WorkModule::on_flag(Flag=FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON)
            }
        });
    }
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6 {
		acmd!(lua_state,{
		if (is_excute) {
			ArticleModule::change_motion(*FIGHTER_PICKEL_GENERATE_ARTICLE_SCARIER, Hash40::new("win_1"), false, -1.0)
			}
 		});
    }
    else {
        acmd!(lua_state, {
			if(is_excute){
				WorkModule::set_int(FIGHTER_PICKEL_CRAFT_WEAPON_KIND_PICK, FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND)
				}
			frame(Frame=119)
			if(is_excute){
				WorkModule::on_flag(Flag=FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON)
			}
		});
	}
}

////////////////////////////////////////////////////////////////////Game\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\





////////////////////////////////////////////////////////////////////Status\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\
#[status_script(agent = "pickel", status = FIGHTER_STATUS_KIND_DEAD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn dead_sound(fighter: &mut L2CFighterCommon){
	let lua_state = fighter.lua_state_agent;
	if  WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_DEAD_MODE) == 0
		||
		WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_DEAD_MODE) == 3	{
			smash_script::macros::PLAY_SE(fighter, Hash40::new("se_pickel_dead"));
		}
	if	WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_DEAD_MODE) == 1	{
			smash_script::macros::PLAY_SE(fighter, Hash40::new("se_pickel_star"));

		}
	if	WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_DEAD_MODE) == 2{
			smash_script::macros::PLAY_SE(fighter, Hash40::new("se_pickel_screen"));
		
		}
		fighter.status_Dead();
}
////////////////////////////////////////////////////////////////////Status\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\








//////////////////////////////////////////////////////////////Scripts being installed\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\
pub fn install(){
	smashline::install_acmd_scripts!(
////////////////////////////////////////////////////////////////////Sounds\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\
	appealhi_sound,
	entry_sound,
	damage_sound,
	appeals_sound,
	appeallw_sound,
	win2_sound,
////////////////////////////////////////////////////////////////////Sounds\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\	
///////////////////////////////////////////////////////////////////Effects\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\	
	entry_effect,
	appeallw_effect,
	appealhi_effect,
	appealsr_effect,
	visualscene_effect,
	win2_effect,
///////////////////////////////////////////////////////////////////Effects\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\	
////////////////////////////////////////////////////////////////////Game\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\	
	appeallw_game,
	entry_game,
////////////////////////////////////////////////////////////////////Game\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\	

	);
	install_status_scripts!(
////////////////////////////////////////////////////////////////////Status\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\
	dead_sound,
////////////////////////////////////////////////////////////////////Status\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\
	);
}
