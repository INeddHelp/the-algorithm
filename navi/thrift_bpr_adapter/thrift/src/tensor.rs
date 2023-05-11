// Autogenerated by Thrift Compiler (0.17.0)
// DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING

#![allow(unused_imports)]
#![allow(unused_extern_crates)]
#![allow(clippy::too_many_arguments, clippy::type_complexity, clippy::vec_box)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::convert::{From, TryFrom};
use std::default::Default;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use thrift::OrderedFloat;
use thrift::{ApplicationError, ApplicationErrorKind, ProtocolError, ProtocolErrorKind, TThriftClient};
use thrift::protocol::{TFieldIdentifier, TListIdentifier, TMapIdentifier, TMessageIdentifier, TMessageType, TInputProtocol, TOutputProtocol, TSerializable, TSetIdentifier, TStructIdentifier, TType};
use thrift::protocol::field_id;
use thrift::protocol::verify_expected_message_type;
use thrift::protocol::verify_expected_sequence_number;
use thrift::protocol::verify_expected_service_call;
use thrift::protocol::verify_required_field_exists;
use thrift::server::TProcessor;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DataType(pub i32);

impl DataType {
  pub const FLOAT: DataType = DataType(0);
  pub const DOUBLE: DataType = DataType(1);
  pub const INT32: DataType = DataType(2);
  pub const INT64: DataType = DataType(3);
  pub const UINT8: DataType = DataType(4);
  pub const STRING: DataType = DataType(5);
  pub const BYTE: DataType = DataType(6);
  pub const BOOL: DataType = DataType(7);
  pub const RESERVED_1: DataType = DataType(8);
  pub const RESERVED_2: DataType = DataType(9);
  pub const RESERVED_3: DataType = DataType(10);
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::FLOAT,
    Self::DOUBLE,
    Self::INT32,
    Self::INT64,
    Self::UINT8,
    Self::STRING,
    Self::BYTE,
    Self::BOOL,
    Self::RESERVED_1,
    Self::RESERVED_2,
    Self::RESERVED_3,
  ];
}

impl TSerializable for DataType {
  #[allow(clippy::trivially_copy_pass_by_ref)]
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    o_prot.write_i32(self.0)
  }
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<DataType> {
    let enum_value = i_prot.read_i32()?;
    Ok(DataType::from(enum_value))
  }
}

impl From<i32> for DataType {
  fn from(i: i32) -> Self {
    match i {
      0 => DataType::FLOAT,
      1 => DataType::DOUBLE,
      2 => DataType::INT32,
      3 => DataType::INT64,
      4 => DataType::UINT8,
      5 => DataType::STRING,
      6 => DataType::BYTE,
      7 => DataType::BOOL,
      8 => DataType::RESERVED_1,
      9 => DataType::RESERVED_2,
      10 => DataType::RESERVED_3,
      _ => DataType(i)
    }
  }
}

impl From<&i32> for DataType {
  fn from(i: &i32) -> Self {
    DataType::from(*i)
  }
}

impl From<DataType> for i32 {
  fn from(e: DataType) -> i32 {
    e.0
  }
}

impl From<&DataType> for i32 {
  fn from(e: &DataType) -> i32 {
    e.0
  }
}

//
// StringTensor
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StringTensor {
  pub strings: Vec<String>,
  pub shape: Option<Vec<i64>>,
}

impl StringTensor {
  pub fn new<F2>(strings: Vec<String>, shape: F2) -> StringTensor where F2: Into<Option<Vec<i64>>> {
    StringTensor {
      strings,
      shape: shape.into(),
    }
  }
}

impl TSerializable for StringTensor {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<StringTensor> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<Vec<String>> = None;
    let mut f_2: Option<Vec<i64>> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<String> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_0 = i_prot.read_string()?;
            val.push(list_elem_0);
          }
          i_prot.read_list_end()?;
          f_1 = Some(val);
        },
        2 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<i64> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_1 = i_prot.read_i64()?;
            val.push(list_elem_1);
          }
          i_prot.read_list_end()?;
          f_2 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("StringTensor.strings", &f_1)?;
    let ret = StringTensor {
      strings: f_1.expect("auto-generated code should have checked for presence of required fields"),
      shape: f_2,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("StringTensor");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("strings", TType::List, 1))?;
    o_prot.write_list_begin(&TListIdentifier::new(TType::String, self.strings.len() as i32))?;
    for e in &self.strings {
      o_prot.write_string(e)?;
    }
    o_prot.write_list_end()?;
    o_prot.write_field_end()?;
    if let Some(ref fld_var) = self.shape {
      o_prot.write_field_begin(&TFieldIdentifier::new("shape", TType::List, 2))?;
      o_prot.write_list_begin(&TListIdentifier::new(TType::I64, fld_var.len() as i32))?;
      for e in fld_var {
        o_prot.write_i64(*e)?;
      }
      o_prot.write_list_end()?;
      o_prot.write_field_end()?
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// Int32Tensor
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Int32Tensor {
  pub ints: Vec<i32>,
  pub shape: Option<Vec<i64>>,
}

impl Int32Tensor {
  pub fn new<F2>(ints: Vec<i32>, shape: F2) -> Int32Tensor where F2: Into<Option<Vec<i64>>> {
    Int32Tensor {
      ints,
      shape: shape.into(),
    }
  }
}

impl TSerializable for Int32Tensor {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<Int32Tensor> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<Vec<i32>> = None;
    let mut f_2: Option<Vec<i64>> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<i32> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_2 = i_prot.read_i32()?;
            val.push(list_elem_2);
          }
          i_prot.read_list_end()?;
          f_1 = Some(val);
        },
        2 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<i64> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_3 = i_prot.read_i64()?;
            val.push(list_elem_3);
          }
          i_prot.read_list_end()?;
          f_2 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("Int32Tensor.ints", &f_1)?;
    let ret = Int32Tensor {
      ints: f_1.expect("auto-generated code should have checked for presence of required fields"),
      shape: f_2,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("Int32Tensor");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("ints", TType::List, 1))?;
    o_prot.write_list_begin(&TListIdentifier::new(TType::I32, self.ints.len() as i32))?;
    for e in &self.ints {
      o_prot.write_i32(*e)?;
    }
    o_prot.write_list_end()?;
    o_prot.write_field_end()?;
    if let Some(ref fld_var) = self.shape {
      o_prot.write_field_begin(&TFieldIdentifier::new("shape", TType::List, 2))?;
      o_prot.write_list_begin(&TListIdentifier::new(TType::I64, fld_var.len() as i32))?;
      for e in fld_var {
        o_prot.write_i64(*e)?;
      }
      o_prot.write_list_end()?;
      o_prot.write_field_end()?
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// Int64Tensor
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Int64Tensor {
  pub longs: Vec<i64>,
  pub shape: Option<Vec<i64>>,
}

impl Int64Tensor {
  pub fn new<F2>(longs: Vec<i64>, shape: F2) -> Int64Tensor where F2: Into<Option<Vec<i64>>> {
    Int64Tensor {
      longs,
      shape: shape.into(),
    }
  }
}

impl TSerializable for Int64Tensor {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<Int64Tensor> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<Vec<i64>> = None;
    let mut f_2: Option<Vec<i64>> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<i64> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_4 = i_prot.read_i64()?;
            val.push(list_elem_4);
          }
          i_prot.read_list_end()?;
          f_1 = Some(val);
        },
        2 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<i64> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_5 = i_prot.read_i64()?;
            val.push(list_elem_5);
          }
          i_prot.read_list_end()?;
          f_2 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("Int64Tensor.longs", &f_1)?;
    let ret = Int64Tensor {
      longs: f_1.expect("auto-generated code should have checked for presence of required fields"),
      shape: f_2,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("Int64Tensor");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("longs", TType::List, 1))?;
    o_prot.write_list_begin(&TListIdentifier::new(TType::I64, self.longs.len() as i32))?;
    for e in &self.longs {
      o_prot.write_i64(*e)?;
    }
    o_prot.write_list_end()?;
    o_prot.write_field_end()?;
    if let Some(ref fld_var) = self.shape {
      o_prot.write_field_begin(&TFieldIdentifier::new("shape", TType::List, 2))?;
      o_prot.write_list_begin(&TListIdentifier::new(TType::I64, fld_var.len() as i32))?;
      for e in fld_var {
        o_prot.write_i64(*e)?;
      }
      o_prot.write_list_end()?;
      o_prot.write_field_end()?
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// FloatTensor
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FloatTensor {
  pub floats: Vec<OrderedFloat<f64>>,
  pub shape: Option<Vec<i64>>,
}

impl FloatTensor {
  pub fn new<F2>(floats: Vec<OrderedFloat<f64>>, shape: F2) -> FloatTensor where F2: Into<Option<Vec<i64>>> {
    FloatTensor {
      floats,
      shape: shape.into(),
    }
  }
}

impl TSerializable for FloatTensor {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<FloatTensor> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<Vec<OrderedFloat<f64>>> = None;
    let mut f_2: Option<Vec<i64>> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<OrderedFloat<f64>> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_6 = OrderedFloat::from(i_prot.read_double()?);
            val.push(list_elem_6);
          }
          i_prot.read_list_end()?;
          f_1 = Some(val);
        },
        2 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<i64> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_7 = i_prot.read_i64()?;
            val.push(list_elem_7);
          }
          i_prot.read_list_end()?;
          f_2 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("FloatTensor.floats", &f_1)?;
    let ret = FloatTensor {
      floats: f_1.expect("auto-generated code should have checked for presence of required fields"),
      shape: f_2,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("FloatTensor");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("floats", TType::List, 1))?;
    o_prot.write_list_begin(&TListIdentifier::new(TType::Double, self.floats.len() as i32))?;
    for e in &self.floats {
      o_prot.write_double((*e).into())?;
    }
    o_prot.write_list_end()?;
    o_prot.write_field_end()?;
    if let Some(ref fld_var) = self.shape {
      o_prot.write_field_begin(&TFieldIdentifier::new("shape", TType::List, 2))?;
      o_prot.write_list_begin(&TListIdentifier::new(TType::I64, fld_var.len() as i32))?;
      for e in fld_var {
        o_prot.write_i64(*e)?;
      }
      o_prot.write_list_end()?;
      o_prot.write_field_end()?
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// DoubleTensor
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DoubleTensor {
  pub doubles: Vec<OrderedFloat<f64>>,
  pub shape: Option<Vec<i64>>,
}

impl DoubleTensor {
  pub fn new<F2>(doubles: Vec<OrderedFloat<f64>>, shape: F2) -> DoubleTensor where F2: Into<Option<Vec<i64>>> {
    DoubleTensor {
      doubles,
      shape: shape.into(),
    }
  }
}

impl TSerializable for DoubleTensor {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<DoubleTensor> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<Vec<OrderedFloat<f64>>> = None;
    let mut f_2: Option<Vec<i64>> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<OrderedFloat<f64>> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_8 = OrderedFloat::from(i_prot.read_double()?);
            val.push(list_elem_8);
          }
          i_prot.read_list_end()?;
          f_1 = Some(val);
        },
        2 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<i64> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_9 = i_prot.read_i64()?;
            val.push(list_elem_9);
          }
          i_prot.read_list_end()?;
          f_2 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("DoubleTensor.doubles", &f_1)?;
    let ret = DoubleTensor {
      doubles: f_1.expect("auto-generated code should have checked for presence of required fields"),
      shape: f_2,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("DoubleTensor");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("doubles", TType::List, 1))?;
    o_prot.write_list_begin(&TListIdentifier::new(TType::Double, self.doubles.len() as i32))?;
    for e in &self.doubles {
      o_prot.write_double((*e).into())?;
    }
    o_prot.write_list_end()?;
    o_prot.write_field_end()?;
    if let Some(ref fld_var) = self.shape {
      o_prot.write_field_begin(&TFieldIdentifier::new("shape", TType::List, 2))?;
      o_prot.write_list_begin(&TListIdentifier::new(TType::I64, fld_var.len() as i32))?;
      for e in fld_var {
        o_prot.write_i64(*e)?;
      }
      o_prot.write_list_end()?;
      o_prot.write_field_end()?
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// BoolTensor
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BoolTensor {
  pub booleans: Vec<bool>,
  pub shape: Option<Vec<i64>>,
}

impl BoolTensor {
  pub fn new<F2>(booleans: Vec<bool>, shape: F2) -> BoolTensor where F2: Into<Option<Vec<i64>>> {
    BoolTensor {
      booleans,
      shape: shape.into(),
    }
  }
}

impl TSerializable for BoolTensor {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<BoolTensor> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<Vec<bool>> = None;
    let mut f_2: Option<Vec<i64>> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<bool> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_10 = i_prot.read_bool()?;
            val.push(list_elem_10);
          }
          i_prot.read_list_end()?;
          f_1 = Some(val);
        },
        2 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<i64> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_11 = i_prot.read_i64()?;
            val.push(list_elem_11);
          }
          i_prot.read_list_end()?;
          f_2 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("BoolTensor.booleans", &f_1)?;
    let ret = BoolTensor {
      booleans: f_1.expect("auto-generated code should have checked for presence of required fields"),
      shape: f_2,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("BoolTensor");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("booleans", TType::List, 1))?;
    o_prot.write_list_begin(&TListIdentifier::new(TType::Bool, self.booleans.len() as i32))?;
    for e in &self.booleans {
      o_prot.write_bool(*e)?;
    }
    o_prot.write_list_end()?;
    o_prot.write_field_end()?;
    if let Some(ref fld_var) = self.shape {
      o_prot.write_field_begin(&TFieldIdentifier::new("shape", TType::List, 2))?;
      o_prot.write_list_begin(&TListIdentifier::new(TType::I64, fld_var.len() as i32))?;
      for e in fld_var {
        o_prot.write_i64(*e)?;
      }
      o_prot.write_list_end()?;
      o_prot.write_field_end()?
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// RawTypedTensor
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RawTypedTensor {
  pub data_type: DataType,
  pub content: Vec<u8>,
  pub shape: Option<Vec<i64>>,
}

impl RawTypedTensor {
  pub fn new<F3>(data_type: DataType, content: Vec<u8>, shape: F3) -> RawTypedTensor where F3: Into<Option<Vec<i64>>> {
    RawTypedTensor {
      data_type,
      content,
      shape: shape.into(),
    }
  }
}

impl TSerializable for RawTypedTensor {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<RawTypedTensor> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<DataType> = None;
    let mut f_2: Option<Vec<u8>> = None;
    let mut f_3: Option<Vec<i64>> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = DataType::read_from_in_protocol(i_prot)?;
          f_1 = Some(val);
        },
        2 => {
          let val = i_prot.read_bytes()?;
          f_2 = Some(val);
        },
        3 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<i64> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_12 = i_prot.read_i64()?;
            val.push(list_elem_12);
          }
          i_prot.read_list_end()?;
          f_3 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("RawTypedTensor.data_type", &f_1)?;
    verify_required_field_exists("RawTypedTensor.content", &f_2)?;
    let ret = RawTypedTensor {
      data_type: f_1.expect("auto-generated code should have checked for presence of required fields"),
      content: f_2.expect("auto-generated code should have checked for presence of required fields"),
      shape: f_3,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("RawTypedTensor");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("dataType", TType::I32, 1))?;
    self.data_type.write_to_out_protocol(o_prot)?;
    o_prot.write_field_end()?;
    o_prot.write_field_begin(&TFieldIdentifier::new("content", TType::String, 2))?;
    o_prot.write_bytes(&self.content)?;
    o_prot.write_field_end()?;
    if let Some(ref fld_var) = self.shape {
      o_prot.write_field_begin(&TFieldIdentifier::new("shape", TType::List, 3))?;
      o_prot.write_list_begin(&TListIdentifier::new(TType::I64, fld_var.len() as i32))?;
      for e in fld_var {
        o_prot.write_i64(*e)?;
      }
      o_prot.write_list_end()?;
      o_prot.write_field_end()?
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// BinaryTensor
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BinaryTensor {
  pub binaries: Vec<Vec<u8>>,
  pub shape: Option<Vec<i64>>,
}

impl BinaryTensor {
  pub fn new<F2>(binaries: Vec<Vec<u8>>, shape: F2) -> BinaryTensor where F2: Into<Option<Vec<i64>>> {
    BinaryTensor {
      binaries,
      shape: shape.into(),
    }
  }
}

impl TSerializable for BinaryTensor {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<BinaryTensor> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<Vec<Vec<u8>>> = None;
    let mut f_2: Option<Vec<i64>> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<Vec<u8>> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_13 = i_prot.read_bytes()?;
            val.push(list_elem_13);
          }
          i_prot.read_list_end()?;
          f_1 = Some(val);
        },
        2 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<i64> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_14 = i_prot.read_i64()?;
            val.push(list_elem_14);
          }
          i_prot.read_list_end()?;
          f_2 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("BinaryTensor.binaries", &f_1)?;
    let ret = BinaryTensor {
      binaries: f_1.expect("auto-generated code should have checked for presence of required fields"),
      shape: f_2,
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("BinaryTensor");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("binaries", TType::List, 1))?;
    o_prot.write_list_begin(&TListIdentifier::new(TType::String, self.binaries.len() as i32))?;
    for e in &self.binaries {
      o_prot.write_bytes(e)?;
    }
    o_prot.write_list_end()?;
    o_prot.write_field_end()?;
    if let Some(ref fld_var) = self.shape {
      o_prot.write_field_begin(&TFieldIdentifier::new("shape", TType::List, 2))?;
      o_prot.write_list_begin(&TListIdentifier::new(TType::I64, fld_var.len() as i32))?;
      for e in fld_var {
        o_prot.write_i64(*e)?;
      }
      o_prot.write_list_end()?;
      o_prot.write_field_end()?
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// GeneralTensor
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GeneralTensor {
  RawTypedTensor(RawTypedTensor),
  StringTensor(StringTensor),
  Int32Tensor(Int32Tensor),
  Int64Tensor(Int64Tensor),
  FloatTensor(FloatTensor),
  DoubleTensor(DoubleTensor),
  BoolTensor(BoolTensor),
  BinaryTensor(BinaryTensor),
}

impl TSerializable for GeneralTensor {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<GeneralTensor> {
    let mut ret: Option<GeneralTensor> = None;
    let mut received_field_count = 0;
    i_prot.read_struct_begin()?;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let val = RawTypedTensor::read_from_in_protocol(i_prot)?;
          if ret.is_none() {
            ret = Some(GeneralTensor::RawTypedTensor(val));
          }
          received_field_count += 1;
        },
        2 => {
          let val = StringTensor::read_from_in_protocol(i_prot)?;
          if ret.is_none() {
            ret = Some(GeneralTensor::StringTensor(val));
          }
          received_field_count += 1;
        },
        3 => {
          let val = Int32Tensor::read_from_in_protocol(i_prot)?;
          if ret.is_none() {
            ret = Some(GeneralTensor::Int32Tensor(val));
          }
          received_field_count += 1;
        },
        4 => {
          let val = Int64Tensor::read_from_in_protocol(i_prot)?;
          if ret.is_none() {
            ret = Some(GeneralTensor::Int64Tensor(val));
          }
          received_field_count += 1;
        },
        5 => {
          let val = FloatTensor::read_from_in_protocol(i_prot)?;
          if ret.is_none() {
            ret = Some(GeneralTensor::FloatTensor(val));
          }
          received_field_count += 1;
        },
        6 => {
          let val = DoubleTensor::read_from_in_protocol(i_prot)?;
          if ret.is_none() {
            ret = Some(GeneralTensor::DoubleTensor(val));
          }
          received_field_count += 1;
        },
        7 => {
          let val = BoolTensor::read_from_in_protocol(i_prot)?;
          if ret.is_none() {
            ret = Some(GeneralTensor::BoolTensor(val));
          }
          received_field_count += 1;
        },
        8 => {
          let val = BinaryTensor::read_from_in_protocol(i_prot)?;
          if ret.is_none() {
            ret = Some(GeneralTensor::BinaryTensor(val));
          }
          received_field_count += 1;
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
          received_field_count += 1;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    if received_field_count == 0 {
      Err(
        thrift::Error::Protocol(
          ProtocolError::new(
            ProtocolErrorKind::InvalidData,
            "received empty union from remote GeneralTensor"
          )
        )
      )
    } else if received_field_count > 1 {
      Err(
        thrift::Error::Protocol(
          ProtocolError::new(
            ProtocolErrorKind::InvalidData,
            "received multiple fields for union from remote GeneralTensor"
          )
        )
      )
    } else {
      Ok(ret.expect("return value should have been constructed"))
    }
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("GeneralTensor");
    o_prot.write_struct_begin(&struct_ident)?;
    match *self {
      GeneralTensor::RawTypedTensor(ref f) => {
        o_prot.write_field_begin(&TFieldIdentifier::new("rawTypedTensor", TType::Struct, 1))?;
        f.write_to_out_protocol(o_prot)?;
        o_prot.write_field_end()?;
      },
      GeneralTensor::StringTensor(ref f) => {
        o_prot.write_field_begin(&TFieldIdentifier::new("stringTensor", TType::Struct, 2))?;
        f.write_to_out_protocol(o_prot)?;
        o_prot.write_field_end()?;
      },
      GeneralTensor::Int32Tensor(ref f) => {
        o_prot.write_field_begin(&TFieldIdentifier::new("int32Tensor", TType::Struct, 3))?;
        f.write_to_out_protocol(o_prot)?;
        o_prot.write_field_end()?;
      },
      GeneralTensor::Int64Tensor(ref f) => {
        o_prot.write_field_begin(&TFieldIdentifier::new("int64Tensor", TType::Struct, 4))?;
        f.write_to_out_protocol(o_prot)?;
        o_prot.write_field_end()?;
      },
      GeneralTensor::FloatTensor(ref f) => {
        o_prot.write_field_begin(&TFieldIdentifier::new("floatTensor", TType::Struct, 5))?;
        f.write_to_out_protocol(o_prot)?;
        o_prot.write_field_end()?;
      },
      GeneralTensor::DoubleTensor(ref f) => {
        o_prot.write_field_begin(&TFieldIdentifier::new("doubleTensor", TType::Struct, 6))?;
        f.write_to_out_protocol(o_prot)?;
        o_prot.write_field_end()?;
      },
      GeneralTensor::BoolTensor(ref f) => {
        o_prot.write_field_begin(&TFieldIdentifier::new("boolTensor", TType::Struct, 7))?;
        f.write_to_out_protocol(o_prot)?;
        o_prot.write_field_end()?;
      },
      GeneralTensor::BinaryTensor(ref f) => {
        o_prot.write_field_begin(&TFieldIdentifier::new("binaryTensor", TType::Struct, 8))?;
        f.write_to_out_protocol(o_prot)?;
        o_prot.write_field_end()?;
      },
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// COOSparseTensor
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct COOSparseTensor {
  pub dense_shape: Vec<i64>,
  pub indices: Int64Tensor,
  pub values: GeneralTensor,
}

impl COOSparseTensor {
  pub fn new(dense_shape: Vec<i64>, indices: Int64Tensor, values: GeneralTensor) -> COOSparseTensor {
    COOSparseTensor {
      dense_shape,
      indices,
      values,
    }
  }
}

impl TSerializable for COOSparseTensor {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<COOSparseTensor> {
    i_prot.read_struct_begin()?;
    let mut f_1: Option<Vec<i64>> = None;
    let mut f_2: Option<Int64Tensor> = None;
    let mut f_3: Option<GeneralTensor> = None;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      match field_id {
        1 => {
          let list_ident = i_prot.read_list_begin()?;
          let mut val: Vec<i64> = Vec::with_capacity(list_ident.size as usize);
          for _ in 0..list_ident.size {
            let list_elem_15 = i_prot.read_i64()?;
            val.push(list_elem_15);
          }
          i_prot.read_list_end()?;
          f_1 = Some(val);
        },
        2 => {
          let val = Int64Tensor::read_from_in_protocol(i_prot)?;
          f_2 = Some(val);
        },
        3 => {
          let val = GeneralTensor::read_from_in_protocol(i_prot)?;
          f_3 = Some(val);
        },
        _ => {
          i_prot.skip(field_ident.field_type)?;
        },
      };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    verify_required_field_exists("COOSparseTensor.dense_shape", &f_1)?;
    verify_required_field_exists("COOSparseTensor.indices", &f_2)?;
    verify_required_field_exists("COOSparseTensor.values", &f_3)?;
    let ret = COOSparseTensor {
      dense_shape: f_1.expect("auto-generated code should have checked for presence of required fields"),
      indices: f_2.expect("auto-generated code should have checked for presence of required fields"),
      values: f_3.expect("auto-generated code should have checked for presence of required fields"),
    };
    Ok(ret)
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("COOSparseTensor");
    o_prot.write_struct_begin(&struct_ident)?;
    o_prot.write_field_begin(&TFieldIdentifier::new("denseShape", TType::List, 1))?;
    o_prot.write_list_begin(&TListIdentifier::new(TType::I64, self.dense_shape.len() as i32))?;
    for e in &self.dense_shape {
      o_prot.write_i64(*e)?;
    }
    o_prot.write_list_end()?;
    o_prot.write_field_end()?;
    o_prot.write_field_begin(&TFieldIdentifier::new("indices", TType::Struct, 2))?;
    self.indices.write_to_out_protocol(o_prot)?;
    o_prot.write_field_end()?;
    o_prot.write_field_begin(&TFieldIdentifier::new("values", TType::Struct, 3))?;
    self.values.write_to_out_protocol(o_prot)?;
    o_prot.write_field_end()?;
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

//
// SparseTensor
//

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SparseTensor {
  CooSparseTensor(COOSparseTensor),
}

impl TSerializable for SparseTensor {
  fn read_from_in_protocol(i_prot: &mut dyn TInputProtocol) -> thrift::Result<SparseTensor> {
    let mut ret: Option<SparseTensor> = None;
    let mut received_field_count = 0;
    i_prot.read_struct_begin()?;
    loop {
      let field_ident = i_prot.read_field_begin()?;
      if field_ident.field_type == TType::Stop {
        break;
      }
      let field_id = field_id(&field_ident)?;
      if let 1 = field_id {
      let val = COOSparseTensor::read_from_in_protocol(i_prot)?;
      if ret.is_none() {
        ret = Some(SparseTensor::CooSparseTensor(val));
      }
      received_field_count += 1;
    } else {
      i_prot.skip(field_ident.field_type)?;
      received_field_count += 1;
    };
      i_prot.read_field_end()?;
    }
    i_prot.read_struct_end()?;
    if received_field_count == 0 {
      Err(
        thrift::Error::Protocol(
          ProtocolError::new(
            ProtocolErrorKind::InvalidData,
            "received empty union from remote SparseTensor"
          )
        )
      )
    } else if received_field_count > 1 {
      Err(
        thrift::Error::Protocol(
          ProtocolError::new(
            ProtocolErrorKind::InvalidData,
            "received multiple fields for union from remote SparseTensor"
          )
        )
      )
    } else {
      Ok(ret.expect("return value should have been constructed"))
    }
  }
  fn write_to_out_protocol(&self, o_prot: &mut dyn TOutputProtocol) -> thrift::Result<()> {
    let struct_ident = TStructIdentifier::new("SparseTensor");
    o_prot.write_struct_begin(&struct_ident)?;
    match *self {
      SparseTensor::CooSparseTensor(ref f) => {
        o_prot.write_field_begin(&TFieldIdentifier::new("cooSparseTensor", TType::Struct, 1))?;
        f.write_to_out_protocol(o_prot)?;
        o_prot.write_field_end()?;
      },
    }
    o_prot.write_field_stop()?;
    o_prot.write_struct_end()
  }
}

