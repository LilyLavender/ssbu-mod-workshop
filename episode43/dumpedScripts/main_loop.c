
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall
L2CFighterMetaknight::status::SpecialNSpin_main_loop
          (L2CFighterMetaknight *this,L2CValue *return_value)

{
  byte bVar1;
  bool bVar2;
  ulong uVar3;
  ulong uVar4;
  L2CValue *pLVar5;
  L2CValue *return_value_00;
  L2CValue *return_value_01;
  L2CValue *return_value_02;
  L2CValue *return_value_03;
  L2CValue *return_value_04;
  L2CValue *return_value_05;
  L2CValue *extraout_x8;
  L2CValue *extraout_x8_00;
  L2CValue *pLVar6;
  float fVar7;
  L2CValue aLStack160 [16];
  L2CValue aLStack144 [16];
  L2CValue aLStack128 [16];
  L2CValue aLStack112 [16];
  L2CValue aLStack96 [16];
  L2CValue LStack80;
  
  lib::L2CValue::L2CValue(aLStack96,0,return_value);
  lib::L2CValue::L2CValue(aLStack112,0,return_value_00);
  fVar7 = (float)app::lua_bind::MotionModule__rate_impl(this->moduleAccessor);
  lib::L2CValue::L2CValue((L2CValue *)&LStack80,fVar7,return_value_01);
  lib::L2CValue::L2CValue(aLStack144,0xf899192aa);
  lib::L2CValue::L2CValue(aLStack160,0xd8ebcb613);
  uVar3 = lib::L2CValue::as_integer(aLStack144);
  uVar4 = lib::L2CValue::as_integer(aLStack160);
  fVar7 = (float)app::lua_bind::WorkModule__get_param_float_impl(this->moduleAccessor,uVar3,uVar4 );
  lib::L2CValue::L2CValue(aLStack128,fVar7,return_value_02);
  uVar3 = lib::L2CValue::operator<=((L2CValue *)&LStack80,aLStack128);
  lib::L2CValue::~L2CValue(aLStack128);
  lib::L2CValue::~L2CValue(aLStack160);
  lib::L2CValue::~L2CValue(aLStack144);
  lib::L2CValue::~L2CValue((L2CValue *)&LStack80);
  if ((uVar3 & 1) != 0) {
    lib::L2CValue::L2CValue
              (aLStack128,_FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_END,
               (L2CValue *)&LUA_SCRIPT_LINE_MAX);
    lib::L2CValue::L2CValue(aLStack144,false);
    lua2cpp::L2CFighterBase::change_status(this,(L2CValue)0x80,(L2CValue)0x70);
    lib::L2CValue::~L2CValue(aLStack144);
    lib::L2CValue::~L2CValue(aLStack128);
  }
  bVar1 = app::lua_bind::StatusModule__is_changing_impl(this->moduleAccessor);
  lib::L2CValue::L2CValue((L2CValue *)&LStack80,(bool)(bVar1 & 1));
  lib::L2CValue::operator=(aLStack96,(L2CValue *)&LStack80);
  lib::L2CValue::~L2CValue((L2CValue *)&LStack80);
  lib::L2CValue::L2CValue((L2CValue *)&LStack80,false);
  lib::L2CValue::operator=(aLStack112,(L2CValue *)&LStack80);
  lib::L2CValue::~L2CValue((L2CValue *)&LStack80);
  lib::L2CValue::operator!(aLStack96,&LStack80);
  bVar2 = lib::L2CValue::operator.cast.to.bool((L2CValue *)&LStack80);
  lib::L2CValue::~L2CValue((L2CValue *)&LStack80);
  if ((bVar2 & 1U) != 0) {
    pLVar6 = &this->globalTable;
    pLVar5 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)pLVar6,0x17);
    lib::L2CValue::L2CValue((L2CValue *)&LStack80,_SITUATION_KIND_GROUND,return_value_03);
    uVar3 = lib::L2CValue::operator==(pLVar5,(L2CValue *)&LStack80);
    lib::L2CValue::~L2CValue((L2CValue *)&LStack80);
    if ((uVar3 & 1) == 0) {
      pLVar5 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)pLVar6,0x16);
      lib::L2CValue::L2CValue((L2CValue *)&LStack80,_SITUATION_KIND_GROUND,return_value_05);
      uVar3 = lib::L2CValue::operator==(pLVar5,(L2CValue *)&LStack80);
      lib::L2CValue::~L2CValue((L2CValue *)&LStack80);
      if ((uVar3 & 1) == 0) goto LAB_710001dc60;
      lib::L2CValue::L2CValue((L2CValue *)&LStack80,true);
      lib::L2CValue::operator=(aLStack112,(L2CValue *)&LStack80);
    }
    else {
      pLVar5 = (L2CValue *)lib::L2CValue::operator[]((L2CValue *)pLVar6,0x16);
      lib::L2CValue::L2CValue((L2CValue *)&LStack80,SITUATION_KIND_AIR,return_value_04);
      uVar3 = lib::L2CValue::operator==(pLVar5,(L2CValue *)&LStack80);
      lib::L2CValue::~L2CValue((L2CValue *)&LStack80);
      if ((uVar3 & 1) == 0) goto LAB_710001dc60;
      lib::L2CValue::L2CValue((L2CValue *)&LStack80,true);
      lib::L2CValue::operator=(aLStack112,(L2CValue *)&LStack80);
    }
    lib::L2CValue::~L2CValue((L2CValue *)&LStack80);
  }
LAB_710001dc60:
  bVar2 = lib::L2CValue::operator.cast.to.bool(aLStack112);
  pLVar6 = extraout_x8;
  if ((bVar2 & 1U) != 0) {
    FUN_710001d850(this);
    pLVar6 = extraout_x8_00;
  }
  lib::L2CValue::L2CValue((L2CValue *)return_value,0,pLVar6);
  lib::L2CValue::~L2CValue(aLStack112);
  lib::L2CValue::~L2CValue(aLStack96);
  return;
}

