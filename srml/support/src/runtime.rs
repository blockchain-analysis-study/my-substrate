// Copyright 2018-2019 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

//! Macros to define a runtime. A runtime is basically all your logic running in Substrate,
//! consisting of selected SRML modules and maybe some of your own modules.
//! A lot of supporting logic is automatically generated for a runtime,
//! mostly for to combine data types and metadata of the included modules.

/// Construct a runtime, with the given name and the given modules.
///
/// The parameters here are specific types for Block, NodeBlock and InherentData
/// (TODO: describe the difference between Block and NodeBlock)
///	and the modules that are used by the runtime.
///
/// # Example:
///
/// ```nocompile
/// construct_runtime!(
///     pub enum Runtime with Log(interalIdent: DigestItem<SessionKey>) where
///         Block = Block,
///         NodeBlock = runtime::Block,
///         UncheckedExtrinsic = UncheckedExtrinsic
///     {
///         System: system,
///         Test: test::{default, Log(Test)},
///         Test2: test_with_long_module::{Module},
///
///         // Module with instances
///         Test3_Instance1: test3::<Instance1>::{Module, Call, Storage, Event<T, I>, Config<T, I>, Origin<T, I>},
///         Test3_DefaultInstance: test3::{Module, Call, Storage, Event<T>, Config<T>, Origin<T>},
///     }
/// )
/// ```
///
/// The module `System: system` will expand to `System: system::{Module, Call, Storage, Event<T>, Config<T>}`.
/// The identifier `System` is the name of the module and the lower case identifier `system` is the
/// name of the Rust module/crate for this Substrate module.
///
/// The module `Test: test::{default, Log(Test)}` will expand to
/// `Test: test::{Module, Call, Storage, Event<T>, Config<T>, Log(Test)}`.
///
/// The module `Test2: test_with_long_module::{Module}` will expand to
/// `Test2: test_with_long_module::{Module}`.
///
/// We provide support for the following types in a module:
/// - `Module`
/// - `Call`
/// - `Storage`
/// - `Event` or `Event<T>` (if the event is generic) or `Event<T, I>` (if also over instance)
/// - `Origin` or `Origin<T>` (if the origin is generic) or `Origin<T, I>` (if also over instance)
/// - `Config` or `Config<T>` (if the config is generic) or `Config<T, I>` (if also over instance)
/// - `Log( $(IDENT),* )`
/// - `Inherent $( (CALL) )*` - If the module provides/can check inherents. The optional parameter
///                             is for modules that use a `Call` from a different module as
///                             inherent.
/// - `ValidateUnsigned`      - If the module validates unsigned extrinsics.
///
/// # Note
///
/// The population of the genesis storage depends on the order of modules. So, if one of your
/// modules depends on another module. The dependent module need to come before the module depending on it.
/*
宏： TODO 这里顺便讲一讲 宏的定义

TODO 定义初始化 runtime 的宏

TODO  这个 宏就TM的看不懂啊
*/
#[macro_export]
macro_rules! construct_runtime {

	// Macro transformations (to convert invocations with incomplete parameters to the canonical
	// form)
    /*
    宏转换（将具有不完整参数的调用转换为规范形式）

    宏中的匹配规则

    宏中使用字面量匹配的哦

    几种指示符：

    ident: 标识符，用来表示函数或变量名
    expr: 表达式
    block: 代码块，用花括号包起来的多个语句
    pat: 模式，普通模式匹配（非宏本身的模式）中的模式，例如 Some(t), (3, 'a', _)
    path: 路径，注意这里不是操作系统中的文件路径，而是用双冒号分隔的限定名(qualified name)，如 std::cmp::PartialOrd
    tt: 单个语法树
    ty: 类型，语义层面的类型，如 i32, char
    item: 条目，
    meta: 元条目
    stmt: 单条语句，如 let a = 42;


    #[macro_use] 表示子模块的宏可以被父模块调用
    #[macro_export] 表示可以被其他的 crate 调用
    $crate 在宏中表示该模块


    */

    /*
    TODO【第一个匹配分支】
    */

	(   // 这里定义一个 enum 的变量名： runtime ，ident代表runtime变量为一个函数或者变量名
	    // 变量： log_internal 为一个函数或者一个变量名
	    // 以及一个名叫做DigestItem<类型1, 类型2, 类型3, ...(这里可能是多个不定数目的类型)>
	    // + 表示一次或多次（至少一次），而 * 表示零次或多次
	    // 重复的模式需要用括号括起来，外面再加上 $，例如 $(...)*, $(...)+
	    //
	    // 注意： 需要说明的是这里的括号和宏里面其它地方一样都可以是三种括号中的任意一种，
	    // 因为括号在这里仅仅是用来标记一个模式的开始和结束，大部分情况重复的模式是用逗号或分号分隔的，
	    // 所以你会经常看到 $(...),*, $(...);*, $(...),+, $(...);+ 这样的用来表示重复。
		pub enum $runtime:ident with Log ($log_internal:ident: DigestItem<$( $log_genarg:ty ),+>)
			where

			    // 这里是一个函数或者变量名
				Block = $block:ident,
				// 这里是一个类型标识
				NodeBlock = $node_block:ty,
				// 这里是一个函数或者变量名
				UncheckedExtrinsic = $uncheckedextrinsic:ident
		{
		    // 表示这里是多个 语法树, 其实就是语法流程
			$( $rest:tt )*
		}

		// 下面 => {}; 中的 是该宏的真正 逻辑块
		//  下面这些是真看不懂啊
		//
		// $crate::construct_runtime! 这个是
		//
		// 定义一个可以在[库内外]都能用的宏。这个函数名字会展开为::construct_runtime 或::mylib::construct_runtime。
		// 为了保证这个系统简单和正确，#[macro_use] extern crate ...应只出现在你包装箱的根中，而不是在mod中
	    // TODO 注意，宏定义中不能多 空格，学一学 yaml 的定义就知道了
        //
	    // TODO 这个表示  库内外 都可任意使用的 宏
        // 这个函数名字会展开为::increment或::mylib::increment。
        // 为了保证这个系统简单和正确，#[macro_use] extern crate ...【应只出现在你包装箱的根中】，而不是在mod中
	    //
	) => {
		$crate::construct_runtime!(
			{
				$runtime;
				$block;
				$node_block;
				$uncheckedextrinsic;
				$log_internal < $( $log_genarg ),* >;
			};
			{};
			$( $rest )*
		);
	};


    /*
    TODO【第二个匹配分支】
    */
	(
		{ $( $preset:tt )* };
		{ $( $expanded:tt )* };
		$name:ident: $module:ident,
		$( $rest:tt )*
	) => {
		$crate::construct_runtime!(
			{ $( $preset )* };
			{ $( $expanded )* $name: $module::{Module, Call, Storage, Event<T>, Config<T>}, };
			$( $rest )*
		);
	};


	/*
    TODO【第三个匹配分支】
    */
	(
		{ $( $preset:tt )* };
		{ $( $expanded:tt )* };
		$name:ident: $module:ident::{
			default,
			$(
				$modules:ident
					$( <$modules_generic:ident $(, $modules_instance:ident)?> )*
					$( ( $( $modules_args:ident ),* ) )*
			),*
		},
		$( $rest:tt )*
	) => {
		$crate::construct_runtime!(
			{ $( $preset )* };
			{
				$( $expanded )*
				$name: $module::{
					Module, Call, Storage, Event<T>, Config<T>,
					$(
						$modules $( <$modules_generic $(, $modules_instance)?> )*
						$( ( $( $modules_args ),* ) )*
					),*
				},
			};
			$( $rest )*
		);
	};


	/*
    TODO【第四个匹配分支】
    */
	(
		{ $( $preset:tt )* };
		{ $( $expanded:tt )* };
		$name:ident: $module:ident::{
			$(
				$modules:ident
					$( <$modules_generic:ident> )*
					$( ( $( $modules_args:ident ),* ) )*
			),*
		},
		$( $rest:tt )*
	) => {
		$crate::construct_runtime!(
			{ $( $preset )* };
			{
				$( $expanded )*
				$name: $module::{
					$(
						$modules $( <$modules_generic> )*
						$( ( $( $modules_args ),* ) )*
					),*
				},
			};
			$( $rest )*
		);
	};

	/*
    TODO【第五个匹配分支】
    */
	( // Instance module: we indicate the generic instance `I` with the full instance path
		{ $( $preset:tt )* };
		{ $( $expanded:tt )* };
		$name:ident: $module:ident ::< $module_instance:ident >::{
			$(
				$modules:ident
					$( <$modules_generic:ident $(, $modules_instance:ident )?> )*
					$( ( $( $modules_args:ident ),* ) )*
			),*
		},
		$( $rest:tt )*
	) => {
		$crate::construct_runtime!(
			{ $( $preset )* };
			{
				$( $expanded )*
				$name: $module::<$module_instance>::{
					$(
						$modules $( <$modules_generic $(, $modules_instance=$module::$module_instance)?> )*
						$( ( $( $modules_args ),* ) )*
					),*
				},
			};
			$( $rest )*
		);
	};

	// The main macro expansion that actually renders the Runtime code.


    /*
    TODO【第六个匹配分支】
    */
	(
		{
			$runtime:ident;
			$block:ident;
			$node_block:ty;
			$uncheckedextrinsic:ident;
			$log_internal:ident <$( $log_genarg:ty ),+>;
		};
		{
			$(
				$name:ident: $module:ident :: $( < $module_instance:ident >:: )? {
					$(
						$modules:ident
							$( <$modules_generic:ident $(, I=$modules_instance:path)?> )*
							$( ( $( $modules_args:ident ),* ) )*
					),*
				},
			)*
		};
	) => {
		#[derive(Clone, Copy, PartialEq, Eq)]
		#[cfg_attr(feature = "std", derive(Debug))]
		pub struct $runtime;
		impl $crate::runtime_primitives::traits::GetNodeBlockType for $runtime {
			type NodeBlock = $node_block;
		}
		impl $crate::runtime_primitives::traits::GetRuntimeBlockType for $runtime {
			type RuntimeBlock = $block;
		}
		$crate::__decl_instance_import!(
			$( $( $module < $module_instance > )? )*
		);
		$crate::__decl_outer_event!(
			$runtime;
			$(
				$name: $module:: $( < $module_instance >:: )? { $( $modules $( <$modules_generic $(, $modules_instance)?> )* ),* }
			),*
		);
		$crate::__decl_outer_origin!(
			$runtime;
			$(
				$name: $module:: $( < $module_instance >:: )? { $( $modules $( <$modules_generic $(, $modules_instance)?> )* ),* }
			),*
		);
		$crate::__decl_all_modules!(
			$runtime;
			;
			{};
			$(
				$name: $module:: $( < $module_instance >:: )? { $( $modules ),* },
			)*
		);
		$crate::__decl_outer_dispatch!(
			$runtime;
			;
			$(
				$name: $module::{ $( $modules ),* }
			),*;
		);
		$crate::__decl_runtime_metadata!(
			$runtime;
			{};
			$(
				$name: $module:: $( < $module_instance >:: )? { $( $modules )* }
			)*
		);
		$crate::__decl_outer_log!(
			$runtime;
			$log_internal < $( $log_genarg ),* >;
			{};
			$(
				$name: $module:: $( < $module_instance >:: )? { $( $modules $( ( $( $modules_args )* ) )* )* }
			)*
		);
		$crate::__decl_outer_config!(
			$runtime;
			{};
			$(
				$name: $module:: $( < $module_instance >:: )? {
					$( $modules $( <$modules_generic $(, $modules_instance)?> )* ),*
				},
			)*
		);
		$crate::__decl_outer_inherent!(
			$runtime;
			$block;
			$uncheckedextrinsic;
			;
			$(
				$name: $module::{ $( $modules $( ( $( $modules_args ),* ) )* ),* }
			),*;
		);
		$crate::__impl_outer_validate_unsigned!(
			$runtime;
			{};
			$(
				$name: $module::{ $( $modules $( ( $( $modules_args )* ) )* )* }
			)*
		);
	}
}

/// A macro that generates a "__decl" private macro that transforms parts of the runtime definition
/// to feed them into a public "impl" macro which accepts the format
/// "pub enum $name for $runtime where system = $system".
///
/// Used to define Event and Origin associated types.
#[macro_export]
#[doc(hidden)]
macro_rules! __create_decl_macro {
	(
		// Parameter $d is a hack for the following issue:
		// https://github.com/rust-lang/rust/issues/35853
		$macro_name:ident, $macro_outer_name:ident, $macro_enum_name:ident, $d:tt
	) => {
		#[macro_export]
		#[doc(hidden)]
		macro_rules! $macro_name {
			(
				$runtime:ident;
				$d( $name:ident : $module:ident:: $d( < $module_instance:ident >:: )? {
					$d( $modules:ident $d( <$modules_generic:ident $d(, $modules_instance:path)?> ),* ),*
				}),*
			) => {
				$d crate::$macro_name!(@inner
					$runtime;
					;
					{};
					$d(
						$name: $module:: $d( < $module_instance >:: )? {
							$d( $modules $d( <$modules_generic $d(, $modules_instance)?> )* ),*
						},
					)*
				);
			};
			(@inner
				$runtime:ident;
				; // there can not be multiple `System`s
				{ $d( $parsed:tt )* };
				System: $module:ident::{
					$d( $modules:ident $d( <$modules_generic:ident> )* ),*
				},
				$d( $rest:tt )*
			) => {
				$d crate::$macro_name!(@inner
					$runtime;
					$module;
					{ $d( $parsed )* };
					$d( $rest )*
				);
			};
			(@inner
				$runtime:ident;
				$d( $system:ident )?;
				{ $d( $parsed:tt )* };
				$name:ident : $module:ident:: < $module_instance:ident >:: {
					$macro_enum_name <$event_generic:ident, $event_instance:path> $d(, $ignore:ident $d( <$ignor:ident $d(, $ignore_instance:path)?> )* )*
				},
				$d( $rest:tt )*
			) => {
				$d crate::$macro_name!(@inner
					$runtime;
					$d( $system )?;
					{
						$d( $parsed )*
						$module $module_instance <$event_generic, $event_instance>,
					};
					$d( $rest )*
				);
			};
			(@inner
				$runtime:ident;
				$d( $system:ident )?;
				{ $d( $parsed:tt )* };
				$name:ident : $module:ident:: < $module_instance:ident >:: {
					$macro_enum_name $d( <$event_generic:ident> )* $d(, $ignore:ident $d( <$ignor:ident $d(, $ignore_instance:path)?> )* )*
				},
				$d( $rest:tt )*
			) => {
				compile_error!{concat!{
					"Module `", stringify!{$name}, "` must have `", stringify!{$macro_enum_name}, "<T, I>`",
					" but has `", stringify!{$macro_enum_name} $d(, "<", stringify!{$event_generic}, ">")*, "`",
					": Instantiated modules must have ", stringify!{$macro_enum_name},
					" generic over instance to be able to convert to outer ", stringify!{$macro_enum_name}
				}}
			};
			(@inner
				$runtime:ident;
				$d( $system:ident )?;
				{ $d( $parsed:tt )* };
				$name:ident : $module:ident:: {
					$macro_enum_name $d( <$event_generic:ident $d(, $event_instance:path)?> )* $d(, $ignore:ident $d( <$ignor:ident $d(, $ignore_instance:path)?> )* )*
				},
				$d( $rest:tt )*
			) => {
				$d crate::$macro_name!(@inner
					$runtime;
					$d( $system )?;
					{
						$d( $parsed )*
						$module $d( <$event_generic $d(, $event_instance)?> )*,
					};
					$d( $rest )*
				);
			};
			(@inner
				$runtime:ident;
				$d( $system:ident )?;
				{ $d( $parsed:tt )* };
				$name:ident : $module:ident:: $d( < $module_instance:ident >:: )? {
					$ignore:ident $d( <$ignor:ident $d(, $ignore_instance:path)?> )* $d(, $modules:ident $d( <$modules_generic:ident $d(, $modules_instance:path)?> )* )*
				},
				$d( $rest:tt )*
			) => {
				$d crate::$macro_name!(@inner
					$runtime;
					$d( $system )?;
					{ $d( $parsed )* };
					$name: $module:: $d( < $module_instance >:: )? { $d( $modules $d( <$modules_generic $d(, $modules_instance)?> )* ),* },
					$d( $rest )*
				);
			};
			(@inner
				$runtime:ident;
				$d( $system:ident )?;
				{ $d( $parsed:tt )* };
				$name:ident: $module:ident:: $d( < $module_instance:ident >:: )? {},
				$d( $rest:tt )*
			) => {
				$d crate::$macro_name!(@inner
					$runtime;
					$d( $system )?;
					{ $d( $parsed )* };
					$d( $rest )*
				);
			};
			(@inner
				$runtime:ident;
				$system:ident;
				{ $d( $parsed_modules:ident $d( $instance:ident )? $d( <$parsed_generic:ident $d(, $parsed_instance_full_path:path)?> )* ,)* };
			) => {
				$d crate::paste::item! {
					$d crate::$macro_outer_name! {

						pub enum $macro_enum_name for $runtime where system = $system {
							$d(
								[< $parsed_modules $d(_ $instance )? >] $d( <$parsed_generic $d(, $parsed_instance_full_path)?> )*,
							)*
						}
					}
				}
			}
		}
	}
}

__create_decl_macro!(__decl_outer_event, impl_outer_event, Event, $);
__create_decl_macro!(__decl_outer_origin, impl_outer_origin, Origin, $);

/// A macro that defines all modules as an associated types of the Runtime type.
#[macro_export]
#[doc(hidden)]
macro_rules! __decl_all_modules {
	(
		$runtime:ident;
		;
		{ $( $parsed:tt )* };
		System: $module:ident::{ Module $(, $modules:ident )* },
		$( $rest:tt )*
	) => {
		$crate::__decl_all_modules!(
			$runtime;
			$module;
			{ $( $parsed )* };
			$( $rest )*
		);
	};
	(
		$runtime:ident;
		$( $system:ident )?;
		{ $( $parsed:tt )* };
		$name:ident: $module:ident:: $( < $module_instance:ident >:: )? { Module $(, $modules:ident )* },
		$( $rest:tt )*
	) => {
		$crate::__decl_all_modules!(
			$runtime;
			$( $system )?;
			{
				$( $parsed )*
				$module::$name $(<$module_instance>)?,
			};
			$( $rest )*
		);
	};
	(
		$runtime:ident;
		$( $system:ident )?;
		{ $( $parsed:tt )* };
		$name:ident: $module:ident:: $( < $module_instance:ident >:: )? { $ignore:ident $(, $modules:ident )* },
		$( $rest:tt )*
	) => {
		$crate::__decl_all_modules!(
			$runtime;
			$( $system )?;
			{ $( $parsed )* };
			$name: $module::{ $( $modules ),* },
			$( $rest )*
		);
	};
	(
		$runtime:ident;
		$( $system:ident )?;
		{ $( $parsed:tt )* };
		$name:ident: $module:ident:: $( < $module_instance:ident >:: )? {},
		$( $rest:tt )*
	) => {
		$crate::__decl_all_modules!(
			$runtime;
			$( $system )?;
			{ $( $parsed )* };
			$( $rest )*
		);
	};
	(
		$runtime:ident;
		$system:ident;
		{ $( $parsed_module:ident :: $parsed_name:ident $(<$instance:ident>)? ,)*};
	) => {
		pub type System = system::Module<$runtime>;
		$(
			pub type $parsed_name = $parsed_module::Module<$runtime $(, $parsed_module::$instance )?>;
		)*
		type AllModules = ( $( $parsed_name, )* );
	}
}

/// A macro that defines the Call enum to represent calls to functions in the modules included
/// in the runtime (by wrapping the values of all FooModule::Call enums).
#[macro_export]
#[doc(hidden)]
macro_rules! __decl_outer_dispatch {
	(
		$runtime:ident;
		$( $parsed_modules:ident :: $parsed_name:ident ),*;
		System: $module:ident::{
			$ignore:ident $( <$ignor:ident> )* $(, $modules:ident $( <$modules_generic:ident> )* )*
		}
		$(, $rest_name:ident : $rest_module:ident::{
			$( $rest_modules:ident $( <$rest_modules_generic:ident> )* ),*
		})*;
	) => {
		$crate::__decl_outer_dispatch!(
			$runtime;
			$( $parsed_modules :: $parsed_name ),*;
			$(
				$rest_name: $rest_module::{
					$( $rest_modules $( <$rest_modules_generic> )* ),*
				}
			),*;
		);
	};
	(
		$runtime:ident;
		$( $parsed_modules:ident :: $parsed_name:ident ),*;
		$name:ident: $module:ident::{
			Call $(, $modules:ident $( <$modules_generic:ident> )* )*
		}
		$(, $rest_name:ident : $rest_module:ident::{
			$( $rest_modules:ident $( <$rest_modules_generic:ident> )* ),*
		})*;
	) => {
		$crate::__decl_outer_dispatch!(
			$runtime;
			$( $parsed_modules :: $parsed_name, )* $module::$name;
			$(
				$rest_name: $rest_module::{
					$( $rest_modules $( <$rest_modules_generic> )* ),*
				}
			),*;
		);
	};
	(
		$runtime:ident;
		$( $parsed_modules:ident :: $parsed_name:ident ),*;
		$name:ident: $module:ident::{
			$ignore:ident $( <$ignor:ident> )* $(, $modules:ident $( <$modules_generic:ident> )* )*
		}
		$(, $rest_name:ident : $rest_module:ident::{
			$( $rest_modules:ident $( <$rest_modules_generic:ident> )* ),*
		})*;
	) => {
		$crate::__decl_outer_dispatch!(
			$runtime;
			$( $parsed_modules :: $parsed_name ),*;
			$name: $module::{ $( $modules $( <$modules_generic> )* ),* }
			$(
				, $rest_name: $rest_module::{
					$( $rest_modules $( <$rest_modules_generic> )* ),*
				}
			)*;
		);
	};
	(
		$runtime:ident;
		$( $parsed_modules:ident :: $parsed_name:ident ),*;
		$name:ident: $module:ident::{}
		$(, $rest_name:ident : $rest_module:ident::{
			$( $rest_modules:ident $( <$rest_modules_generic:ident> )* ),*
		})*;
	) => {
		$crate::__decl_outer_dispatch!(
			$runtime;
			$( $parsed_modules :: $parsed_name ),*;
			$(
				$rest_name: $rest_module::{
					$( $rest_modules $( <$rest_modules_generic> )* ),*
				}
			),*;
		);
	};
	(
		$runtime:ident;
		$( $parsed_modules:ident :: $parsed_name:ident ),*;
		;
	) => {
		$crate::impl_outer_dispatch!(
			pub enum Call for $runtime where origin: Origin {
				$( $parsed_modules::$parsed_name, )*
			}
		);
	};
}

/// A private macro that generates metadata() method for the runtime. See impl_runtime_metadata macro.
#[macro_export]
#[doc(hidden)]
macro_rules! __decl_runtime_metadata {
	// leading is Module : parse
	(
		$runtime:ident;
		{ $( $parsed:tt )* };
		$( { leading_module: $( $leading_module:ident )* } )?
		$name:ident: $module:ident:: $( < $module_instance:ident >:: )? {
			Module $( $modules:ident )*
		}
		$( $rest:tt )*
	) => {
		$crate::__decl_runtime_metadata!(
			$runtime;
			{
				$( $parsed )*
				$module $( < $module_instance > )?  { $( $( $leading_module )* )? $( $modules )* }
			};
			$( $rest )*
		);
	};
	// leading isn't Module : put it in leadings
	(
		$runtime:ident;
		{ $( $parsed:tt )* };
		$( { leading_module: $( $leading_module:ident )* } )?
		$name:ident: $module:ident:: $( < $module_instance:ident >:: )? {
			$other_module:ident $( $modules:ident )*
		}
		$( $rest:tt )*
	) => {
		$crate::__decl_runtime_metadata!(
			$runtime;
			{ $( $parsed )* };
			{ leading_module: $( $( $leading_module )* )? $other_module }
			$name: $module:: $( < $module_instance >:: )? {
				$( $modules )*
			}
			$( $rest )*
		);
	};
	// does not contain Module : skip
	(
		$runtime:ident;
		{ $( $parsed:tt )* };
		$( { leading_module: $( $leading_module:ident )* } )?
		$name:ident: $module:ident:: $( < $module_instance:ident >:: )? {}
		$( $rest:tt )*
	) => {
		$crate::__decl_runtime_metadata!(
			$runtime;
			{ $( $parsed )* };
			$( $rest )*
		);
	};
	// end of decl
	(
		$runtime:ident;
		{ $( $parsed_modules:ident $( < $module_instance:ident > )? { $( $withs:ident )* } )* };
	) => {
		$crate::impl_runtime_metadata!(
			for $runtime with modules
				$( $parsed_modules::Module $( < $module_instance > )? with $( $withs )* , )*
		);
	}

}
/// A private macro that generates Log enum for the runtime. See impl_outer_log macro.
#[macro_export]
#[doc(hidden)]
macro_rules! __decl_outer_log {
	(
		$runtime:ident;
		$log_internal:ident <$( $log_genarg:ty ),+>;
		{ $( $parsed:tt )* };
		$name:ident: $module:ident:: $(<$module_instance:ident>::)? {
			Log ( $( $args:ident )* ) $( $modules:ident $( ( $( $modules_args:ident )* ) )* )*
		}
		$( $rest:tt )*
	) => {
		$crate::__decl_outer_log!(
			$runtime;
			$log_internal < $( $log_genarg ),* >;
			{ $( $parsed )* $module $(<$module_instance>)? ( $( $args )* )};
			$( $rest )*
		);
	};
	(
		$runtime:ident;
		$log_internal:ident <$( $log_genarg:ty ),+>;
		{ $( $parsed:tt )* };
		$name:ident: $module:ident:: $(<$module_instance:ident>::)? {
			$ignore:ident $( ( $( $args_ignore:ident )* ) )*
			$( $modules:ident $( ( $( $modules_args:ident )* ) )* )*
		}
		$( $rest:tt )*
	) => {
		$crate::__decl_outer_log!(
			$runtime;
			$log_internal < $( $log_genarg ),* >;
			{ $( $parsed )* };
			$name: $module:: $(<$module_instance>::)? { $( $modules $( ( $( $modules_args )* ) )* )* }
			$( $rest )*
		);
	};
	(
		$runtime:ident;
		$log_internal:ident <$( $log_genarg:ty ),+>;
		{ $( $parsed:tt )* };
		$name:ident: $module:ident:: $(<$module_instance:ident>::)? {}
		$( $rest:tt )*
	) => {
		$crate::__decl_outer_log!(
			$runtime;
			$log_internal < $( $log_genarg ),* >;
			{ $( $parsed )* };
			$( $rest )*
		);
	};
	(
		$runtime:ident;
		$log_internal:ident <$( $log_genarg:ty ),+>;
		{ $(
			$parsed_modules:ident $(< $parsed_instance:ident >)? ( $( $parsed_args:ident )* )
		)* };
	) => {
		$crate::paste::item! {
			$crate::runtime_primitives::impl_outer_log!(
				pub enum Log($log_internal: DigestItem<$( $log_genarg ),*>) for $runtime {
					$( [< $parsed_modules $(_ $parsed_instance)? >] $(< $parsed_modules::$parsed_instance >)? ( $( $parsed_args ),* ) ),*
				}
			);
		}
	};
}

/// A private macro that generates GenesisConfig for the runtime. See impl_outer_config macro.
#[macro_export]
#[doc(hidden)]
macro_rules! __decl_outer_config {
	(
		$runtime:ident;
		{ $( $parsed:tt )* };
		$name:ident: $module:ident:: $( < $module_instance:ident >:: )? {
			Config $(< $config_generic:ident $(, $config_instance:path)?>)? $(, $modules:ident $( <$modules_generic:ident $(, $modules_instance:path)?> )* )*
		},
		$( $rest:tt )*
	) => {
		$crate::__decl_outer_config!(
			$runtime;
			{
				$( $parsed )*
				$module::$name $( $module_instance )?  $(<$config_generic $(, $config_instance)?>)?,
			};
			$( $rest )*
		);
	};
	(
		$runtime:ident;
		{ $( $parsed:tt )* };
		$name:ident: $module:ident:: $( < $module_instance:ident >:: )? {
			$ignore:ident $( <$ignor:ident $(, $ignore_instance:path)?> )* $(, $modules:ident $( <$modules_generic:ident $(, $modules_instance:path)?> )* )*
		},
		$( $rest:tt )*
	) => {
		$crate::__decl_outer_config!(
			$runtime;
			{ $( $parsed )* };
			$name: $module:: $( < $module_instance >:: )? { $( $modules $( <$modules_generic $(, $modules_instance)?> )* ),* },
			$( $rest )*
		);
	};
	(
		$runtime:ident;
		{ $( $parsed:tt )* };
		$name:ident: $module:ident:: $( < $module_instance:ident >:: )? {},
		$( $rest:tt )*
	) => {
		$crate::__decl_outer_config!(
			$runtime;
			{ $( $parsed )* };
			$( $rest )*
		);
	};
	(
		$runtime:ident;
		{$( $parsed_modules:ident :: $parsed_name:ident $( $parsed_instance:ident )?  $( < $parsed_generic:ident $(, $parsed_instance_full_path:path)? > )* ,)* };
	) => {
		$crate::paste::item! {
			$crate::runtime_primitives::impl_outer_config!(
				pub struct GenesisConfig for $runtime {
					$(
						[< $parsed_name Config >] => [< $parsed_modules $( _ $parsed_instance)? >] $( < $parsed_generic $(, $parsed_instance_full_path)? > )*,
					)*
				}
			);
		}
	};
}

/// A private macro that generates check_inherents() implementation for the runtime.
#[macro_export]
#[doc(hidden)]
macro_rules! __decl_outer_inherent {
	(
		$runtime:ident;
		$block:ident;
		$uncheckedextrinsic:ident;
		$( $parsed_name:ident :: $parsed_call:ident ),*;
		$name:ident: $module:ident::{
			Inherent $(, $modules:ident $( ( $( $modules_call:ident )* ) )* )*
		}
		$(, $rest_name:ident : $rest_module:ident::{
			$( $rest_modules:ident $( ( $( $rest_call:ident )* ) )* ),*
		})*;
	) => {
		$crate::__decl_outer_inherent!(
			$runtime;
			$block;
			$uncheckedextrinsic;
			$( $parsed_name :: $parsed_call, )* $name::$name;
			$(
				$rest_name: $rest_module::{
					$( $rest_modules $( ( $( $rest_call )* ) )* ),*
				}
			),*;
		);
	};
	(
		$runtime:ident;
		$block:ident;
		$uncheckedextrinsic:ident;
		$( $parsed_name:ident :: $parsed_call:ident ),*;
		$name:ident: $module:ident::{
			Inherent ( $call:ident ) $(, $modules:ident $( ( $( $modules_call:ident )* ) )* )*
		}
		$(, $rest_name:ident : $rest_module:ident::{
			$( $rest_modules:ident $( ( $( $rest_call:ident )* ) )* ),*
		})*;
	) => {
		$crate::__decl_outer_inherent!(
			$runtime;
			$block;
			$uncheckedextrinsic;
			$( $parsed_name :: $parsed_call, )* $name::$call;
			$(
				$rest_name: $rest_module::{
					$( $rest_modules $( ( $( $rest_call )* ) )* ),*
				}
			),*;
		);
	};
	(
		$runtime:ident;
		$block:ident;
		$uncheckedextrinsic:ident;
		$( $parsed_name:ident :: $parsed_call:ident ),*;
		$name:ident: $module:ident::{
			$ignore:ident $( ( $( $ignor:ident )* ) )*
				$(, $modules:ident $( ( $( $modules_call:ident )* ) )* )*
		}
		$(, $rest_name:ident : $rest_module:ident::{
			$( $rest_modules:ident $( ( $( $rest_call:ident )* ) )* ),*
		})*;
	) => {
		$crate::__decl_outer_inherent!(
			$runtime;
			$block;
			$uncheckedextrinsic;
			$( $parsed_name :: $parsed_call ),*;
			$name: $module::{ $( $modules $( ( $( $modules_call )* ) )* ),* }
			$(
				, $rest_name: $rest_module::{
					$( $rest_modules $( ( $( $rest_call )* ) )* ),*
				}
			)*;
		);
	};
	(
		$runtime:ident;
		$block:ident;
		$uncheckedextrinsic:ident;
		$( $parsed_name:ident :: $parsed_call:ident ),*;
		$name:ident: $module:ident::{}
		$(, $rest_name:ident : $rest_module:ident::{
			$( $rest_modules:ident $( ( $( $rest_call:ident )* ) )* ),*
		})*;
	) => {
		$crate::__decl_outer_inherent!(
			$runtime;
			$block;
			$uncheckedextrinsic;
			$( $parsed_name :: $parsed_call ),*;
			$(
				$rest_name: $rest_module::{
					$( $rest_modules $( ( $( $rest_call )* ) )* ),*
				}
			),*;
		);
	};
	(
		$runtime:ident;
		$block:ident;
		$uncheckedextrinsic:ident;
		$( $parsed_name:ident :: $parsed_call:ident ),*;
		;
	) => {
		$crate::impl_outer_inherent!(
			impl Inherents where Block = $block, UncheckedExtrinsic = $uncheckedextrinsic {
				$( $parsed_name : $parsed_call, )*
			}
		);
	};
}

#[macro_export]
#[doc(hidden)]
// Those imports are used by event, config, origin and log macros to get access to its inner type
macro_rules! __decl_instance_import {
	( $( $module:ident <$instance:ident> )* ) => {
		$crate::paste::item! {
			$(use $module as [< $module _ $instance >];)*
		}
	};
}

/// A private macro that calls impl_outer_validate_unsigned for Call.
#[macro_export]
#[doc(hidden)]
macro_rules! __impl_outer_validate_unsigned {
	(
		$runtime:ident;
		{ $( $parsed:tt )* };
		$name:ident: $module:ident:: $(<$module_instance:ident>::)? {
			ValidateUnsigned $( $modules:ident $( ( $( $modules_args:ident )* ) )* )*
		}
		$( $rest:tt )*
	) => {
		$crate::__impl_outer_validate_unsigned!(
			$runtime;
			{ $( $parsed )* $name };
			$( $rest )*
		);
	};
	(
		$runtime:ident;
		{ $( $parsed:tt )* };
		$name:ident: $module:ident:: $(<$module_instance:ident>::)? {
			$ignore:ident $( ( $( $args_ignore:ident )* ) )*
			$( $modules:ident $( ( $( $modules_args:ident )* ) )* )*
		}
		$( $rest:tt )*
	) => {
		$crate::__impl_outer_validate_unsigned!(
			$runtime;
			{ $( $parsed )* };
			$name: $module:: $(<$module_instance>::)? {
				$( $modules $( ( $( $modules_args )* ) )* )*
			}
			$( $rest )*
		);
	};
	(
		$runtime:ident;
		{ $( $parsed:tt )* };
		$name:ident: $module:ident:: $(<$module_instance:ident>::)? {}
		$( $rest:tt )*
	) => {
		$crate::__impl_outer_validate_unsigned!(
			$runtime;
			{ $( $parsed )* };
			$( $rest )*
		);
	};
	(
		$runtime:ident;
		{ $(
			$parsed_modules:ident
		)* };
	) => {
		$crate::impl_outer_validate_unsigned!(
			impl ValidateUnsigned for $runtime {
				$( $parsed_modules )*
			}
		);
	};
}
