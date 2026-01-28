#![feature(lazy_cell, ptr_sub_ptr)]

use unity::prelude::*;
use engage::gamedata::unit::Unit;
use engage::gamesound::GameSound;
use engage::gamedata::skill::SkillArray;
use engage::gamemessage::GameMessage;
use engage::sequence::unitgrowsequence::UnitGrowSequence;
use engage::mess::Mess;

use std::path::Path;
use std::fs::read_to_string;

const SKILL_1_LEVEL: u8 = 0;
const SKILL_2_LEVEL: u8 = 5;
const SKILL_3_LEVEL: u8 = 10;
const SKILL_4_LEVEL: u8 = 15;
const SKILL_5_LEVEL: u8 = 20;
const SKILL_6_LEVEL: u8 = 25;
const SKILL_7_LEVEL: u8 = 30;
const SKILL_8_LEVEL: u8 = 35;
const SKILL_9_LEVEL: u8 = 40;


#[unity::hook("App", "UnitGrowSequence", "LevelUp")]
pub fn levelup_checknewlearnskills(this: &UnitGrowSequence, method_info: OptionalMethod) {
    call_original!(this, method_info);

    let unit = this.unit;
    let path = Path::new("sd:/engage/config/morelearnskills/learnskills.txt");
    let new_learn_skills: String = read_to_string(path).expect("REASON");
    let unit_jid = unit.job.name.to_string();
    let start_bytes = new_learn_skills.find(&unit_jid);
    if start_bytes.is_none() {return};
    let start_bytes_0 = start_bytes.unwrap() + unit_jid.len();
    let end_bytes = new_learn_skills[start_bytes_0..].find("END");
    if end_bytes.is_none() {return};
    let end_bytes_0 = end_bytes.unwrap() + start_bytes_0;
    let job_learn_skills = &new_learn_skills[start_bytes_0..end_bytes_0];

    let mut current_level = unit.level;
    if unit.get_job().is_high() {current_level += 20};
    if current_level >= SKILL_1_LEVEL {
        let start_bytes_1 = job_learn_skills.find("|1|");
        if start_bytes_1.is_none() {return};
        let start_bytes_0_1 = start_bytes_1.unwrap() + 3;
        let end_bytes_1 = job_learn_skills.find("|2|");
        if end_bytes_1.is_none() {return};
        let end_bytes_0_1 = end_bytes_1.unwrap();
        let learn_skill_1 = &job_learn_skills[start_bytes_0_1..end_bytes_0_1];
        if learn_skill_1 == "" {return};
        if unit.private_skill.find_sid(learn_skill_1).is_some() {return};
        unit.private_skill.add_sid(learn_skill_1, 2, 0);
        learn_message(this, unit, learn_skill_1);
    };
    if current_level >= SKILL_2_LEVEL {
        let start_bytes_2 = job_learn_skills.find("|2|");
        if start_bytes_2.is_none() {return};
        let start_bytes_0_2 = start_bytes_2.unwrap() + 3;
        let end_bytes_2 = job_learn_skills.find("|3|");
        if end_bytes_2.is_none() {return};
        let end_bytes_0_2 = end_bytes_2.unwrap();
        let learn_skill_2 = &job_learn_skills[start_bytes_0_2..end_bytes_0_2];
        if learn_skill_2 == "" {return};
        if unit.private_skill.find_sid(learn_skill_2).is_some() {return};
        unit.private_skill.add_sid(learn_skill_2, 2, 0);
        learn_message(this, unit, learn_skill_2);
    };
    if current_level >= SKILL_3_LEVEL {
        let start_bytes_3 = job_learn_skills.find("|3|");
        if start_bytes_3.is_none() {return};
        let start_bytes_0_3 = start_bytes_3.unwrap() + 3;
        let end_bytes_3 = job_learn_skills.find("|4|");
        if end_bytes_3.is_none() {return};
        let end_bytes_0_3 = end_bytes_3.unwrap();
        let learn_skill_3 = &job_learn_skills[start_bytes_0_3..end_bytes_0_3];
        if learn_skill_3 == "" {return};
        if unit.private_skill.find_sid(learn_skill_3).is_some() {return};
        unit.private_skill.add_sid(learn_skill_3, 2, 0);
        learn_message(this, unit, learn_skill_3);
    };
    if current_level >= SKILL_4_LEVEL {
        let start_bytes_4 = job_learn_skills.find("|4|");
        if start_bytes_4.is_none() {return};
        let start_bytes_0_4 = start_bytes_4.unwrap() + 3;
        let end_bytes_4 = job_learn_skills.find("|5|");
        if end_bytes_4.is_none() {return};
        let end_bytes_0_4 = end_bytes_4.unwrap();
        let learn_skill_4 = &job_learn_skills[start_bytes_0_4..end_bytes_0_4];
        if learn_skill_4 == "" {return};
        if unit.private_skill.find_sid(learn_skill_4).is_some() {return};
        unit.private_skill.add_sid(learn_skill_4, 2, 0);
        learn_message(this, unit, learn_skill_4);
    };
    if current_level >= SKILL_5_LEVEL {
        let start_bytes_5 = job_learn_skills.find("|5|");
        if start_bytes_5.is_none() {return};
        let start_bytes_0_5 = start_bytes_5.unwrap() + 3;
        let end_bytes_5 = job_learn_skills.find("|6|");
        if end_bytes_5.is_none() {return};
        let end_bytes_0_5 = end_bytes_5.unwrap();
        let learn_skill_5 = &job_learn_skills[start_bytes_0_5..end_bytes_0_5];
        if learn_skill_5 == "" {return};
        if unit.private_skill.find_sid(learn_skill_5).is_some() {return};
        unit.private_skill.add_sid(learn_skill_5, 2, 0);
        learn_message(this, unit, learn_skill_5);
    };
    if current_level >= SKILL_6_LEVEL {
        let start_bytes_6 = job_learn_skills.find("|6|");
        if start_bytes_6.is_none() {return};
        let start_bytes_0_6 = start_bytes_6.unwrap() + 3;
        let end_bytes_6 = job_learn_skills.find("|7|");
        if end_bytes_6.is_none() {return};
        let end_bytes_0_6 = end_bytes_6.unwrap();
        let learn_skill_6 = &job_learn_skills[start_bytes_0_6..end_bytes_0_6];
        if learn_skill_6 == "" {return};
        if unit.private_skill.find_sid(learn_skill_6).is_some() {return};
        unit.private_skill.add_sid(learn_skill_6, 2, 0);
        learn_message(this, unit, learn_skill_6);
    };
    if current_level >= SKILL_7_LEVEL {
        let start_bytes_7 = job_learn_skills.find("|7|");
        if start_bytes_7.is_none() {return};
        let start_bytes_0_7 = start_bytes_7.unwrap() + 3;
        let end_bytes_7 = job_learn_skills.find("|8|");
        if end_bytes_7.is_none() {return};
        let end_bytes_0_7 = end_bytes_7.unwrap();
        let learn_skill_7 = &job_learn_skills[start_bytes_0_7..end_bytes_0_7];
        if learn_skill_7 == "" {return};
        if unit.private_skill.find_sid(learn_skill_7).is_some() {return};
        unit.private_skill.add_sid(learn_skill_7, 2, 0);
        learn_message(this, unit, learn_skill_7);
    };
    if current_level >= SKILL_8_LEVEL {
        let start_bytes_8 = job_learn_skills.find("|8|");
        if start_bytes_8.is_none() {return};
        let start_bytes_0_8 = start_bytes_8.unwrap() + 3;
        let end_bytes_8 = job_learn_skills.find("|9|");
        if end_bytes_8.is_none() {return};
        let end_bytes_0_8 = end_bytes_8.unwrap();
        let learn_skill_8 = &job_learn_skills[start_bytes_0_8..end_bytes_0_8];
        if learn_skill_8 == "" {return};
        if unit.private_skill.find_sid(learn_skill_8).is_some() {return};
        unit.private_skill.add_sid(learn_skill_8, 2, 0);
        learn_message(this, unit, learn_skill_8);
    };
    if current_level >= SKILL_9_LEVEL {
        let start_bytes_9 = job_learn_skills.find("|9|");
        if start_bytes_9.is_none() {return};
        let start_bytes_0_9 = start_bytes_9.unwrap() + 3;
        let end_bytes_9 = job_learn_skills.find("|10|");
        if end_bytes_9.is_none() {return};
        let end_bytes_0_9 = end_bytes_9.unwrap();
        let learn_skill_9 = &job_learn_skills[start_bytes_0_9..end_bytes_0_9];
        if learn_skill_9 == "" {return};
        if unit.private_skill.find_sid(learn_skill_9).is_some() {return};
        unit.private_skill.add_sid(learn_skill_9, 2, 0);
        learn_message(this, unit, learn_skill_9);
    };
}

pub fn learn_message(this: &UnitGrowSequence, this_unit: &Unit, sid: &str) {
    let name = this_unit.get_pid().to_string();
    let m_name = Mess::get_name(name).to_string();
    let current = SkillArray::find_sid(this_unit.private_skill, sid);
    let current_skill = current.unwrap().fields.name;
    let current_skill_name = current_skill.expect("REASON").to_string();
    let m_current_skill_name = Mess::get_name(current_skill_name).to_string();

    let message = format!("{m_name} learnt {m_current_skill_name}");

    GameSound::post_event("ItemGet_Important", None);
    GameMessage::create_key_wait(this, message);
}

#[unity::hook("App", "UnitGrowSequence", "ClassChange")]
pub fn classchange_clearandaddlearnskills(this: &UnitGrowSequence, method_info: OptionalMethod) {
    let old_jid = this.unit.job.name.to_string();
    let new_jid = this.class_change_job.unwrap().name.to_string();
    let unit = this.unit;
    let path = Path::new("sd:/engage/config/morelearnskills/learnskills.txt");
    let new_learn_skills: String = read_to_string(path).expect("REASON");

    let old_start_bytes = new_learn_skills.find(&old_jid);
    if old_start_bytes.is_some() {
        let start_bytes_0 = old_start_bytes.unwrap() + old_jid.len();
        let end_bytes = new_learn_skills[start_bytes_0..].find("END");
        if end_bytes.is_none() {return};
        let end_bytes_0 = end_bytes.unwrap() + start_bytes_0;
        let job_learn_skills = &new_learn_skills[start_bytes_0..end_bytes_0];

        let mut current_level = unit.level;
        if unit.get_job().is_high() {current_level += 20};
        if current_level >= SKILL_1_LEVEL {
            let start_bytes_1 = job_learn_skills.find("|1|");
            if start_bytes_1.is_none() {return};
            let start_bytes_0_1 = start_bytes_1.unwrap() + 3;
            let end_bytes_1 = job_learn_skills.find("|2|");
            if end_bytes_1.is_none() {return};
            let end_bytes_0_1 = end_bytes_1.unwrap();
            let learn_skill_1 = &job_learn_skills[start_bytes_0_1..end_bytes_0_1];
            if learn_skill_1 == "" {return};
            if unit.private_skill.find_sid(learn_skill_1).is_none() {return};
            unit.private_skill.remove_sid(learn_skill_1.into());
        };
        if current_level >= SKILL_2_LEVEL {
            let start_bytes_2 = job_learn_skills.find("|2|");
            if start_bytes_2.is_none() {return};
            let start_bytes_0_2 = start_bytes_2.unwrap() + 3;
            let end_bytes_2 = job_learn_skills.find("|3|");
            if end_bytes_2.is_none() {return};
            let end_bytes_0_2 = end_bytes_2.unwrap();
            let learn_skill_2 = &job_learn_skills[start_bytes_0_2..end_bytes_0_2];
            if learn_skill_2 == "" {return};
            if unit.private_skill.find_sid(learn_skill_2).is_none() {return};
            unit.private_skill.remove_sid(learn_skill_2.into());
        };
        if current_level >= SKILL_3_LEVEL {
            let start_bytes_3 = job_learn_skills.find("|3|");
            if start_bytes_3.is_none() {return};
            let start_bytes_0_3 = start_bytes_3.unwrap() + 3;
            let end_bytes_3 = job_learn_skills.find("|4|");
            if end_bytes_3.is_none() {return};
            let end_bytes_0_3 = end_bytes_3.unwrap();
            let learn_skill_3 = &job_learn_skills[start_bytes_0_3..end_bytes_0_3];
            if learn_skill_3 == "" {return};
            if unit.private_skill.find_sid(learn_skill_3).is_none() {return};
            unit.private_skill.remove_sid(learn_skill_3.into());
        };
        if current_level >= SKILL_4_LEVEL {
            let start_bytes_4 = job_learn_skills.find("|4|");
            if start_bytes_4.is_none() {return};
            let start_bytes_0_4 = start_bytes_4.unwrap() + 3;
            let end_bytes_4 = job_learn_skills.find("|5|");
            if end_bytes_4.is_none() {return};
            let end_bytes_0_4 = end_bytes_4.unwrap();
            let learn_skill_4 = &job_learn_skills[start_bytes_0_4..end_bytes_0_4];
            if learn_skill_4 == "" {return};
            if unit.private_skill.find_sid(learn_skill_4).is_none() {return};
            unit.private_skill.remove_sid(learn_skill_4.into());
        };
        if current_level >= SKILL_5_LEVEL {
            let start_bytes_5 = job_learn_skills.find("|5|");
            if start_bytes_5.is_none() {return};
            let start_bytes_0_5 = start_bytes_5.unwrap() + 3;
            let end_bytes_5 = job_learn_skills.find("|6|");
            if end_bytes_5.is_none() {return};
            let end_bytes_0_5 = end_bytes_5.unwrap();
            let learn_skill_5 = &job_learn_skills[start_bytes_0_5..end_bytes_0_5];
            if learn_skill_5 == "" {return};
            if unit.private_skill.find_sid(learn_skill_5).is_none() {return};
            unit.private_skill.remove_sid(learn_skill_5.into());
        };
        if current_level >= SKILL_6_LEVEL {
            let start_bytes_6 = job_learn_skills.find("|6|");
            if start_bytes_6.is_none() {return};
            let start_bytes_0_6 = start_bytes_6.unwrap() + 3;
            let end_bytes_6 = job_learn_skills.find("|7|");
            if end_bytes_6.is_none() {return};
            let end_bytes_0_6 = end_bytes_6.unwrap();
            let learn_skill_6 = &job_learn_skills[start_bytes_0_6..end_bytes_0_6];
            if learn_skill_6 == "" {return};
            if unit.private_skill.find_sid(learn_skill_6).is_none() {return};
            unit.private_skill.remove_sid(learn_skill_6.into());
        };
        if current_level >= SKILL_7_LEVEL {
            let start_bytes_7 = job_learn_skills.find("|7|");
            if start_bytes_7.is_none() {return};
            let start_bytes_0_7 = start_bytes_7.unwrap() + 3;
            let end_bytes_7 = job_learn_skills.find("|8|");
            if end_bytes_7.is_none() {return};
            let end_bytes_0_7 = end_bytes_7.unwrap();
            let learn_skill_7 = &job_learn_skills[start_bytes_0_7..end_bytes_0_7];
            if learn_skill_7 == "" {return};
            if unit.private_skill.find_sid(learn_skill_7).is_none() {return};
            unit.private_skill.remove_sid(learn_skill_7.into());
        };
        if current_level >= SKILL_8_LEVEL {
            let start_bytes_8 = job_learn_skills.find("|8|");
            if start_bytes_8.is_none() {return};
            let start_bytes_0_8 = start_bytes_8.unwrap() + 3;
            let end_bytes_8 = job_learn_skills.find("|9|");
            if end_bytes_8.is_none() {return};
            let end_bytes_0_8 = end_bytes_8.unwrap();
            let learn_skill_8 = &job_learn_skills[start_bytes_0_8..end_bytes_0_8];
            if learn_skill_8 == "" {return};
            if unit.private_skill.find_sid(learn_skill_8).is_none() {return};
            unit.private_skill.remove_sid(learn_skill_8.into());
        };
        if current_level >= SKILL_9_LEVEL {
            let start_bytes_9 = job_learn_skills.find("|9|");
            if start_bytes_9.is_none() {return};
            let start_bytes_0_9 = start_bytes_9.unwrap() + 3;
            let end_bytes_9 = job_learn_skills.find("|10|");
            if end_bytes_9.is_none() {return};
            let end_bytes_0_9 = end_bytes_9.unwrap();
            let learn_skill_9 = &job_learn_skills[start_bytes_0_9..end_bytes_0_9];
            if learn_skill_9 == "" {return};
            if unit.private_skill.find_sid(learn_skill_9).is_none() {return};
            unit.private_skill.remove_sid(learn_skill_9.into());
        };
    };

    call_original!(this, method_info);

    let new_start_bytes = new_learn_skills.find(&new_jid);
    if new_start_bytes.is_some() {
        let start_bytes_0 = new_start_bytes.unwrap() + new_jid.len();
        let end_bytes = new_learn_skills[start_bytes_0..].find("END");
        if end_bytes.is_none() {return};
        let end_bytes_0 = end_bytes.unwrap() + new_jid.len();
        let job_learn_skills = &new_learn_skills[start_bytes_0..end_bytes_0];

        let mut current_level = unit.level;
        if unit.get_job().is_high() {current_level += 20};
        if current_level >= SKILL_1_LEVEL {
            let start_bytes_1 = job_learn_skills.find("|1|");
            if start_bytes_1.is_none() {return};
            let start_bytes_0_1 = start_bytes_1.unwrap() + 3;
            let end_bytes_1 = job_learn_skills.find("|2|");
            if end_bytes_1.is_none() {return};
            let end_bytes_0_1 = end_bytes_1.unwrap();
            let learn_skill_1 = &job_learn_skills[start_bytes_0_1..end_bytes_0_1];
            if learn_skill_1 == "" {return};
            if unit.private_skill.find_sid(learn_skill_1).is_some() {return};
            unit.private_skill.add_sid(learn_skill_1, 2, 0);
            learn_message(this, unit, learn_skill_1);
        };
        if current_level >= SKILL_2_LEVEL {
            let start_bytes_2 = job_learn_skills.find("|2|");
            if start_bytes_2.is_none() {return};
            let start_bytes_0_2 = start_bytes_2.unwrap() + 3;
            let end_bytes_2 = job_learn_skills.find("|3|");
            if end_bytes_2.is_none() {return};
            let end_bytes_0_2 = end_bytes_2.unwrap();
            let learn_skill_2 = &job_learn_skills[start_bytes_0_2..end_bytes_0_2];
            if learn_skill_2 == "" {return};
            if unit.private_skill.find_sid(learn_skill_2).is_some() {return};
            unit.private_skill.add_sid(learn_skill_2, 2, 0);
            learn_message(this, unit, learn_skill_2);
        };
        if current_level >= SKILL_3_LEVEL {
            let start_bytes_3 = job_learn_skills.find("|3|");
            if start_bytes_3.is_none() {return};
            let start_bytes_0_3 = start_bytes_3.unwrap() + 3;
            let end_bytes_3 = job_learn_skills.find("|4|");
            if end_bytes_3.is_none() {return};
            let end_bytes_0_3 = end_bytes_3.unwrap();
            let learn_skill_3 = &job_learn_skills[start_bytes_0_3..end_bytes_0_3];
            if learn_skill_3 == "" {return};
            if unit.private_skill.find_sid(learn_skill_3).is_some() {return};
            unit.private_skill.add_sid(learn_skill_3, 2, 0);
            learn_message(this, unit, learn_skill_3);
        };
        if current_level >= SKILL_4_LEVEL {
            let start_bytes_4 = job_learn_skills.find("|4|");
            if start_bytes_4.is_none() {return};
            let start_bytes_0_4 = start_bytes_4.unwrap() + 3;
            let end_bytes_4 = job_learn_skills.find("|5|");
            if end_bytes_4.is_none() {return};
            let end_bytes_0_4 = end_bytes_4.unwrap();
            let learn_skill_4 = &job_learn_skills[start_bytes_0_4..end_bytes_0_4];
            if learn_skill_4 == "" {return};
            if unit.private_skill.find_sid(learn_skill_4).is_some() {return};
            unit.private_skill.add_sid(learn_skill_4, 2, 0);
            learn_message(this, unit, learn_skill_4);
        };
        if current_level >= SKILL_5_LEVEL {
            let start_bytes_5 = job_learn_skills.find("|5|");
            if start_bytes_5.is_none() {return};
            let start_bytes_0_5 = start_bytes_5.unwrap() + 3;
            let end_bytes_5 = job_learn_skills.find("|6|");
            if end_bytes_5.is_none() {return};
            let end_bytes_0_5 = end_bytes_5.unwrap();
            let learn_skill_5 = &job_learn_skills[start_bytes_0_5..end_bytes_0_5];
            if learn_skill_5 == "" {return};
            if unit.private_skill.find_sid(learn_skill_5).is_some() {return};
            unit.private_skill.add_sid(learn_skill_5, 2, 0);
            learn_message(this, unit, learn_skill_5);
        };
        if current_level >= SKILL_6_LEVEL {
            let start_bytes_6 = job_learn_skills.find("|6|");
            if start_bytes_6.is_none() {return};
            let start_bytes_0_6 = start_bytes_6.unwrap() + 3;
            let end_bytes_6 = job_learn_skills.find("|7|");
            if end_bytes_6.is_none() {return};
            let end_bytes_0_6 = end_bytes_6.unwrap();
            let learn_skill_6 = &job_learn_skills[start_bytes_0_6..end_bytes_0_6];
            if learn_skill_6 == "" {return};
            if unit.private_skill.find_sid(learn_skill_6).is_some() {return};
            unit.private_skill.add_sid(learn_skill_6, 2, 0);
            learn_message(this, unit, learn_skill_6);
        };
        if current_level >= SKILL_7_LEVEL {
            let start_bytes_7 = job_learn_skills.find("|7|");
            if start_bytes_7.is_none() {return};
            let start_bytes_0_7 = start_bytes_7.unwrap() + 3;
            let end_bytes_7 = job_learn_skills.find("|8|");
            if end_bytes_7.is_none() {return};
            let end_bytes_0_7 = end_bytes_7.unwrap();
            let learn_skill_7 = &job_learn_skills[start_bytes_0_7..end_bytes_0_7];
            if learn_skill_7 == "" {return};
            if unit.private_skill.find_sid(learn_skill_7).is_some() {return};
            unit.private_skill.add_sid(learn_skill_7, 2, 0);
            learn_message(this, unit, learn_skill_7);
        };
        if current_level >= SKILL_8_LEVEL {
            let start_bytes_8 = job_learn_skills.find("|8|");
            if start_bytes_8.is_none() {return};
            let start_bytes_0_8 = start_bytes_8.unwrap() + 3;
            let end_bytes_8 = job_learn_skills.find("|9|");
            if end_bytes_8.is_none() {return};
            let end_bytes_0_8 = end_bytes_8.unwrap();
            let learn_skill_8 = &job_learn_skills[start_bytes_0_8..end_bytes_0_8];
            if learn_skill_8 == "" {return};
            if unit.private_skill.find_sid(learn_skill_8).is_some() {return};
            unit.private_skill.add_sid(learn_skill_8, 2, 0);
            learn_message(this, unit, learn_skill_8);
        };
        if current_level >= SKILL_9_LEVEL {
            let start_bytes_9 = job_learn_skills.find("|9|");
            if start_bytes_9.is_none() {return};
            let start_bytes_0_9 = start_bytes_9.unwrap() + 3;
            let end_bytes_9 = job_learn_skills.find("|10|");
            if end_bytes_9.is_none() {return};
            let end_bytes_0_9 = end_bytes_9.unwrap();
            let learn_skill_9 = &job_learn_skills[start_bytes_0_9..end_bytes_0_9];
            if learn_skill_9 == "" {return};
            if unit.private_skill.find_sid(learn_skill_9).is_some() {return};
            unit.private_skill.add_sid(learn_skill_9, 2, 0);
            learn_message(this, unit, learn_skill_9);
        };
    };


}


#[skyline::main(name = "learnskl")]
pub fn main() {
    skyline::install_hooks!(levelup_checknewlearnskills, classchange_clearandaddlearnskills);
}
