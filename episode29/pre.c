
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall
L2CFighterCharizard::status::SpecialHi_pre(L2CFighterCharizard *this,L2CValue *return_value)

{
byte bVar1;
byte bVar2;
byte bVar3;
byte bVar4;
SituationKind SVar5;
int iVar6;
uint uVar7;
GroundCliffCheckKind GVar8;
int iVar9;
int iVar10;
int iVar11;
uint uVar12;
ulong uVar13;
uint in_stack_fffffffffffffef4;
L2CValue aLStack248 [16];
L2CValue aLStack232 [16];
L2CValue aLStack216 [16];
L2CValue aLStack200 [16];
L2CValue aLStack184 [16];
L2CValue aLStack168 [16];
L2CValue aLStack152 [16];
L2CValue aLStack136 [16];
L2CValue aLStack120 [24];

lib::L2CValue::L2CValue(aLStack120,SITUATION_KIND_NONE);
lib::L2CValue::L2CValue(aLStack136,_FIGHTER_KINETIC_TYPE_UNIQ);
lib::L2CValue::L2CValue(aLStack152,GROUND_CORRECT_KIND_KEEP);
lib::L2CValue::L2CValue(aLStack168,_GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
lib::L2CValue::L2CValue(aLStack184,true);
lib::L2CValue::L2CValue(aLStack200,FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG);
lib::L2CValue::L2CValue(aLStack216,_FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT);
lib::L2CValue::L2CValue(aLStack232,FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT);
lib::L2CValue::L2CValue(aLStack248,0);
SVar5 = lib::L2CValue::as_integer(aLStack120);
iVar6 = lib::L2CValue::as_integer(aLStack136);
uVar7 = lib::L2CValue::as_integer(aLStack152);
GVar8 = lib::L2CValue::as_integer(aLStack168);
bVar1 = lib::L2CValue::as_bool(aLStack184);
iVar9 = lib::L2CValue::as_integer(aLStack200);
iVar10 = lib::L2CValue::as_integer(aLStack216);
iVar11 = lib::L2CValue::as_integer(aLStack232);
lib::L2CValue::as_integer(aLStack248);
app::lua_bind::StatusModule__init_settings_impl
(this->moduleAccessor,SVar5,iVar6,uVar7,GVar8,(bool)(bVar1 & 1),iVar9,iVar10,iVar11,
 in_stack_fffffffffffffef4);
lib::L2CValue::L2CValue(aLStack120,false);
lib::L2CValue::L2CValue(aLStack136,_FIGHTER_TREADED_KIND_NO_REAC);
lib::L2CValue::L2CValue(aLStack152,false);
lib::L2CValue::L2CValue(aLStack168,false);
lib::L2CValue::L2CValue(aLStack184,false);
lib::L2CValue::L2CValue
(aLStack200,
 _FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI |
 FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_O N)
;
lib::L2CValue::L2CValue(aLStack216,_FIGHTER_STATUS_ATTR_START_TURN);
lib::L2CValue::L2CValue(aLStack232,_FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI);
lib::L2CValue::L2CValue(aLStack248,0);
bVar1 = lib::L2CValue::as_bool(aLStack120);
iVar6 = lib::L2CValue::as_integer(aLStack136);
bVar2 = lib::L2CValue::as_bool(aLStack152);
bVar3 = lib::L2CValue::as_bool(aLStack168);
bVar4 = lib::L2CValue::as_bool(aLStack184);
uVar13 = lib::L2CValue::as_integer(aLStack200);
uVar7 = lib::L2CValue::as_integer(aLStack216);
uVar12 = lib::L2CValue::as_integer(aLStack232);
lib::L2CValue::as_integer(aLStack248);
app::lua_bind::FighterStatusModuleImpl__set_fighter_status_data_impl
(this->moduleAccessor,(bool)(bVar1 & 1),iVar6,(bool)(bVar2 & 1),(bool)(bVar3 & 1),
 (bool)(bVar4 & 1),uVar13,uVar7,uVar12,in_stack_fffffffffffffef4);
lib::L2CValue::L2CValue((L2CValue *)return_value,0);
return;
}
