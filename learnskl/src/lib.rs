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

const SKILL_LEVEL: [u8; 9] = [0, 5, 10, 15, 20, 25, 30, 35, 40];


#[unity::hook("App", "UnitGrowSequence", "LevelUp")]
pub fn levelup_checknewlearnskills(this: &UnitGrowSequence, method_info: OptionalMethod) {
    call_original!(this, method_info);

    let unit = this.unit;
    let path = Path::new("sd:/engage/config/morelearnskills/learnskills.txt");
    let new_learn_skills: String = read_to_string(path).expect("REASON");
    let unit_jid = unit.job.name.to_string();
    if let Some(start_bytes) = new_learn_skills.find(&unit_jid) {
        let start_bytes_0 = start_bytes + unit_jid.len();
        if let Some(end_bytes) = new_learn_skills[start_bytes_0..].find("END") {
            let end_bytes_0 = end_bytes + start_bytes_0;
            let job_learn_skills = &new_learn_skills[start_bytes_0..end_bytes_0];
            let mut current_level = unit.level;
            if unit.get_job().is_high() {current_level += 20};
            let mut lvl_pos = 0;
            for lvl in SKILL_LEVEL {
                lvl_pos += 1;
                if current_level >= lvl {
                    let skill_start = format!("|{}|", (lvl_pos).to_string());
                    let skill_end = format!("|{}|", (lvl_pos + 1).to_string());
                    'check_lvl: {
                        if let Some(start_bytes_lvl) = job_learn_skills.find(&skill_start) {
                            if let Some(end_bytes_lvl) = job_learn_skills.find(&skill_end) {
                                let learn_skill_lvl = &job_learn_skills[(start_bytes_lvl + 3)..end_bytes_lvl];
                                if learn_skill_lvl == "" {break 'check_lvl};
                                if unit.private_skill.find_sid(learn_skill_lvl).is_some() {break 'check_lvl};
                                unit.private_skill.add_sid(learn_skill_lvl, 2, 0);
                                learn_message(this, unit, learn_skill_lvl);
                            };
                        };
                    };
                };
            };
        };
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
    if let Some(old_start_bytes) = new_learn_skills.find(&old_jid) {
        let start_bytes_0 = old_start_bytes + old_jid.len();
        if let Some(end_bytes) = new_learn_skills[start_bytes_0..].find("END") {
            let end_bytes_0 = end_bytes + start_bytes_0;
            let job_learn_skills = &new_learn_skills[start_bytes_0..end_bytes_0];
            let mut current_level = unit.level;
            if unit.get_job().is_high() {current_level += 20};
            let mut lvl_pos = 0;
            for lvl in SKILL_LEVEL {
                lvl_pos += 1;
                if current_level >= lvl {
                    let skill_start = format!("|{}|", (lvl_pos).to_string());
                    let skill_end = format!("|{}|", (lvl_pos + 1).to_string());
                    'check_lvl: {
                        if let Some(start_bytes_lvl) = job_learn_skills.find(&skill_start) {
                            if let Some(end_bytes_lvl) = job_learn_skills.find(&skill_end) {
                                let learn_skill_lvl = &job_learn_skills[(start_bytes_lvl + 3)..end_bytes_lvl];
                                if learn_skill_lvl == "" {break 'check_lvl};
                                if unit.private_skill.find_sid(learn_skill_lvl).is_some() {
                                    unit.private_skill.remove_sid(learn_skill_lvl.into());
                                };
                            };
                        };
                    };
                };
            };
        };
    };

    call_original!(this, method_info);

    if let Some(new_start_bytes) = new_learn_skills.find(&new_jid) {
        let start_bytes_0 = new_start_bytes + new_jid.len();
        if let Some(end_bytes) = new_learn_skills[start_bytes_0..].find("END") {
            let end_bytes_0 = end_bytes + start_bytes_0;
            let job_learn_skills = &new_learn_skills[start_bytes_0..end_bytes_0];
            let mut current_level = unit.level;
            if unit.get_job().is_high() {current_level += 20};
            let mut lvl_pos = 0;
            for lvl in SKILL_LEVEL {
                lvl_pos += 1;
                if current_level >= lvl {
                    let skill_start = format!("|{}|", (lvl_pos).to_string());
                    let skill_end = format!("|{}|", (lvl_pos + 1).to_string());
                    'check_lvl: {
                        if let Some(start_bytes_lvl) = job_learn_skills.find(&skill_start) {
                            if let Some(end_bytes_lvl) = job_learn_skills.find(&skill_end) {
                                let learn_skill_lvl = &job_learn_skills[(start_bytes_lvl + 3)..end_bytes_lvl];
                                if learn_skill_lvl == "" {break 'check_lvl};
                                if unit.private_skill.find_sid(learn_skill_lvl).is_some() {break 'check_lvl};
                                unit.private_skill.add_sid(learn_skill_lvl, 2, 0);
                                learn_message(this, unit, learn_skill_lvl);
                            };
                        };
                    };
                };
            };
        };
    };
}


#[skyline::main(name = "learnskl")]
pub fn main() {
    skyline::install_hooks!(levelup_checknewlearnskills, classchange_clearandaddlearnskills);
}
