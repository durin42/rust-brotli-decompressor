use alloc;
use alloc::Allocator;
use core::ops::AddAssign;
pub mod interface;
use super::{HuffmanCode, HuffmanTreeGroup};
use super::huffman::histogram::{ANSTable, HistogramSpec};
pub use self::interface::{
  TrueBoolTrait,
  FalseBoolTrait,
  BoolTrait,
  Unconditional,
  Speculative,
  EntropyEncoder,
  EntropyDecoder,
  SymbolCast,
};
pub mod tee;
pub mod huffman;
pub use self::huffman::{
  HUFFMAN_TABLE_BITS,
  HUFFMAN_TABLE_MASK,
  HuffmanDecoder,
};
#[derive(Default)]
pub struct NopEncoder {
}

#[allow(unused)]
impl<AllocU8:Allocator<u8>,AllocU32: Allocator<u32>> EntropyEncoder<AllocU8, AllocU32> for NopEncoder {
  fn put<Symbol: Sized+Ord+AddAssign<Symbol>+From<u8>+SymbolCast + Clone, AllocS:Allocator<Symbol>, AllocH: Allocator<u32>, Spec:HistogramSpec, Speculative:BoolTrait>(
    &mut self,
    m8: &mut AllocU8, m32: &mut AllocU32,
    group:&[&[HuffmanCode];256],
    prob: &ANSTable<u32, Symbol, AllocS, AllocH, Spec>,
    prior: u8,
    symbol: Symbol,
    is_speculative: Speculative){
  }
  fn put_stationary<Symbol: Sized+Ord+AddAssign<Symbol>+From<u8>+SymbolCast + Clone, AllocS:Allocator<Symbol>, AllocH: Allocator<u32>, Spec:HistogramSpec, Speculative:BoolTrait>(
    &mut self,
    m8: &mut AllocU8, m32: &mut AllocU32,
    group:&[HuffmanCode],
    prob: &ANSTable<u32, Symbol, AllocS, AllocH, Spec>,
    l1numbits: u8,
    symbol: Symbol, 
    speculative: Speculative) {
  }
  fn put_uniform<Speculative:BoolTrait> (
    &mut self,
    m8: &mut AllocU8, m32: &mut AllocU32,
    nbits: u8,
    symbol: u32,
    is_speculative: Speculative) {
  }
  fn begin_speculative(&mut self){}
  fn commit_speculative(&mut self){}
  fn abort_speculative(&mut self){}
  fn drain(&mut self, out_data: &mut [u8]) -> usize {0}
  fn finish(&mut self, out_data:&mut [u8]) -> usize {0}
}

