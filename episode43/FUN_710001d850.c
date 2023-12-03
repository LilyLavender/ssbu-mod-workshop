
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_710001d850(long param_1)

{
  GroundCorrectKind GVar1;
  int iVar2;
  L2CValue *this;
  ulong uVar3;
  L2CValue *return_value;
  L2CValue *return_value_00;
  L2CValue *return_value_01;
  float fVar4;
  L2CValue aLStack80 [16];
  L2CValue aLStack64 [16];
  
  this = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)(param_1 + 200),0x16);
  lib::L2CValue::L2CValue(aLStack64,_SITUATION_KIND_GROUND,return_value);
  uVar3 = lib::L2CValue::operator==(this,aLStack64);
  lib::L2CValue::~L2CValue(aLStack64);
  if ((uVar3 & 1) == 0) {
    lib::L2CValue::L2CValue(aLStack64,GROUND_CORRECT_KIND_AIR,return_value_00);
    GVar1 = lib::L2CValue::as_integer(aLStack64);
    app::lua_bind::GroundModule__correct_impl
              (*(BattleObjectModuleAccessor **)(param_1 + 0x40),GVar1);
    lib::L2CValue::~L2CValue(aLStack64);
    lib::L2CValue::L2CValue
              (aLStack64,_FIGHTER_KINETIC_TYPE_METAKNIGHT_SPECIAL_AIR_N,(L2CValue *)0x4a48);
    iVar2 = lib::L2CValue::as_integer(aLStack64);
    app::lua_bind::KineticModule__change_kinetic_impl
              (*(BattleObjectModuleAccessor **)(param_1 + 0x40),iVar2);
  }
  else {
    lib::L2CValue::L2CValue(aLStack64,GROUND_CORRECT_KIND_GROUND_CLIFF_STOP,return_value_00);
    GVar1 = lib::L2CValue::as_integer(aLStack64);
    app::lua_bind::GroundModule__correct_impl
              (*(BattleObjectModuleAccessor **)(param_1 + 0x40),GVar1);
    lib::L2CValue::~L2CValue(aLStack64);
    lib::L2CValue::L2CValue(aLStack64,_FIGHTER_KINETIC_TYPE_METAKNIGHT_SPECIAL_N,(L2CValue *)0x4a 5c)
    ;
    iVar2 = lib::L2CValue::as_integer(aLStack64);
    app::lua_bind::KineticModule__change_kinetic_impl
              (*(BattleObjectModuleAccessor **)(param_1 + 0x40),iVar2);
    lib::L2CValue::~L2CValue(aLStack64);
    lib::L2CValue::L2CValue(aLStack64,0.0,return_value_01);
    lib::L2CValue::L2CValue
              (aLStack80,_FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_FLOAT_GROUND_EFFECT_COUNTER ,
               (L2CValue *)0x4a7c);
    fVar4 = (float)lib::L2CValue::as_number(aLStack64);
    iVar2 = lib::L2CValue::as_integer(aLStack80);
    app::lua_bind::WorkModule__set_float_impl
              (*(BattleObjectModuleAccessor **)(param_1 + 0x40),fVar4,iVar2);
    lib::L2CValue::~L2CValue(aLStack80);
  }
  lib::L2CValue::~L2CValue(aLStack64);
  return;
}

