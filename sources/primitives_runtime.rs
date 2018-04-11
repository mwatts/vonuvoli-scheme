

use super::builtins::exports::*;
use super::errors::exports::*;
use super::evaluator::exports::*;
use super::primitives_procedures::exports::*;
use super::transcript::exports::*;
use super::values::exports::*;

use super::prelude::*;




pub mod exports {
	
	pub use super::RuntimePrimitive0;
	pub use super::RuntimePrimitive1;
	pub use super::RuntimePrimitive2;
	pub use super::RuntimePrimitive3;
	pub use super::RuntimePrimitive4;
	pub use super::RuntimePrimitive5;
	pub use super::RuntimePrimitiveN;
	pub use super::RuntimePrimitiveV;
	
	pub use super::runtime_primitive_0_evaluate;
	pub use super::runtime_primitive_1_evaluate;
	pub use super::runtime_primitive_2_evaluate;
	pub use super::runtime_primitive_3_evaluate;
	pub use super::runtime_primitive_4_evaluate;
	pub use super::runtime_primitive_5_evaluate;
	pub use super::runtime_primitive_n_evaluate;
	
	pub use super::runtime_primitive_v_alternative_0;
	pub use super::runtime_primitive_v_alternative_1;
	pub use super::runtime_primitive_v_alternative_2;
	pub use super::runtime_primitive_v_alternative_3;
	pub use super::runtime_primitive_v_alternative_4;
	pub use super::runtime_primitive_v_alternative_5;
	pub use super::runtime_primitive_v_alternative_n;
	
	pub use super::runtime_primitive_0_attributes;
	pub use super::runtime_primitive_1_attributes;
	pub use super::runtime_primitive_2_attributes;
	pub use super::runtime_primitive_3_attributes;
	pub use super::runtime_primitive_4_attributes;
	pub use super::runtime_primitive_5_attributes;
	pub use super::runtime_primitive_n_attributes;
	
}




#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum RuntimePrimitive0 {
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	ParameterBuild,
	
	ProcessArgumentsAsList,
	ProcessArgumentsAsArray,
	ProcessEnvironmentVariablesAsList,
	ProcessEnvironmentVariablesAsArray,
	
	ProcessExit,
	ProcessExitEmergency,
	
	PosixTimestamp,
	
	JiffiesTimestamp,
	JiffiesPerSecond,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum RuntimePrimitive1 {
	
	ValueRaise,
	
	ErrorRaise,
	ErrorBuild,
	ErrorMessage,
	ErrorArgumentsAsList,
	ErrorArgumentsAsArray,
	ErrorArgumentsAsValues,
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	ParameterBuild,
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	ParameterResolve,
	
	TranscriptTraceCritical,
	TranscriptTraceError,
	TranscriptTraceWarning,
	TranscriptTraceNotice,
	TranscriptTraceInformation,
	TranscriptTraceInternal,
	TranscriptTraceDebugging,
	
	ProcessArgument,
	ProcessEnvironmentVariable,
	
	ProcessExit,
	ProcessExitEmergency,
	
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessSpawnExtended,
	
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessWaitPoll,
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessWaitTry,
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessWaitCheck,
	
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessStdinGet,
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessStdoutGet,
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessStderrGet,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum RuntimePrimitive2 {
	
	ErrorRaise,
	ErrorBuild,
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	ParameterBuild,
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	ParameterResolve,
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	ParameterConfigure,
	
	TranscriptTraceCritical,
	TranscriptTraceError,
	TranscriptTraceWarning,
	TranscriptTraceNotice,
	TranscriptTraceInformation,
	TranscriptTraceInternal,
	TranscriptTraceDebugging,
	
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessSpawnExtended,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum RuntimePrimitive3 {
	
	ErrorRaise,
	ErrorBuild,
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	ParameterBuild,
	
	TranscriptTraceCritical,
	TranscriptTraceError,
	TranscriptTraceWarning,
	TranscriptTraceNotice,
	TranscriptTraceInformation,
	TranscriptTraceInternal,
	TranscriptTraceDebugging,
	
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessSpawnExtended,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum RuntimePrimitive4 {
	
	ErrorRaise,
	ErrorBuild,
	
	TranscriptTraceCritical,
	TranscriptTraceError,
	TranscriptTraceWarning,
	TranscriptTraceNotice,
	TranscriptTraceInformation,
	TranscriptTraceInternal,
	TranscriptTraceDebugging,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum RuntimePrimitive5 {
	
	ErrorRaise,
	ErrorBuild,
	
	TranscriptTraceCritical,
	TranscriptTraceError,
	TranscriptTraceWarning,
	TranscriptTraceNotice,
	TranscriptTraceInformation,
	TranscriptTraceInternal,
	TranscriptTraceDebugging,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum RuntimePrimitiveN {
	
	ErrorRaise,
	ErrorBuild,
	
	TranscriptTraceCritical,
	TranscriptTraceError,
	TranscriptTraceWarning,
	TranscriptTraceNotice,
	TranscriptTraceInformation,
	TranscriptTraceInternal,
	TranscriptTraceDebugging,
	
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessSpawn,
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessRunTry,
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessRunCheck,
	
}


#[ derive (Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash) ]
pub enum RuntimePrimitiveV {
	
	ErrorRaise,
	ErrorBuild,
	
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	ParameterBuild,
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	ParameterResolve,
	#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
	ParameterConfigure,
	
	TranscriptTraceCritical,
	TranscriptTraceError,
	TranscriptTraceWarning,
	TranscriptTraceNotice,
	TranscriptTraceInformation,
	TranscriptTraceInternal,
	TranscriptTraceDebugging,
	
	ProcessExit,
	ProcessExitEmergency,
	
	#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
	ProcessSpawnExtended,
	
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_0_evaluate (primitive : RuntimePrimitive0, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitive0::ParameterBuild =>
			return parameter_build (None, None, None, None, evaluator) .into_0 (),
		
		RuntimePrimitive0::ProcessArgumentsAsList =>
			return process_arguments (evaluator, false),
		
		RuntimePrimitive0::ProcessArgumentsAsArray =>
			return process_arguments (evaluator, true),
		
		RuntimePrimitive0::ProcessEnvironmentVariablesAsList =>
			return process_environment_variables (evaluator, false),
		
		RuntimePrimitive0::ProcessEnvironmentVariablesAsArray =>
			return process_environment_variables (evaluator, true),
		
		RuntimePrimitive0::ProcessExit =>
			return Err (try! (error_exit (None, false)) .into ()),
		
		RuntimePrimitive0::ProcessExitEmergency =>
			return Err (try! (error_exit (None, true)) .into ()),
		
		RuntimePrimitive0::PosixTimestamp =>
			return posix_timestamp () .into_0 (),
		
		RuntimePrimitive0::JiffiesTimestamp =>
			return jiffies_timestamp () .into_0 (),
		
		RuntimePrimitive0::JiffiesPerSecond =>
			return jiffies_per_second () .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_1_evaluate (primitive : RuntimePrimitive1, input_1 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		RuntimePrimitive1::ValueRaise =>
			return Err (error_coerce (None, input_1)),
		
		RuntimePrimitive1::ErrorRaise =>
			return Err (try! (error_build_0 (None, input_1))),
		
		RuntimePrimitive1::ErrorBuild =>
			return error_build_0 (None, input_1) .into_0 (),
		
		RuntimePrimitive1::ErrorMessage =>
			return error_message (input_1) .into_0 (),
		
		RuntimePrimitive1::ErrorArgumentsAsList =>
			return error_arguments_as_list (input_1) .into_0 (),
		
		RuntimePrimitive1::ErrorArgumentsAsArray =>
			return error_arguments_as_array (input_1) .into_0 (),
		
		RuntimePrimitive1::ErrorArgumentsAsValues =>
			return error_arguments_as_values (input_1) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitive1::ParameterBuild =>
			return parameter_build (None, Some (input_1), None, None, evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitive1::ParameterResolve =>
			return parameter_resolve (input_1, None, evaluator),
		
		RuntimePrimitive1::TranscriptTraceCritical =>
			return transcript_trace_g (TranscriptLevel::Critical, &[input_1], evaluator) .into_0 (),
		
		RuntimePrimitive1::TranscriptTraceError =>
			return transcript_trace_g (TranscriptLevel::Error, &[input_1], evaluator) .into_0 (),
		
		RuntimePrimitive1::TranscriptTraceWarning =>
			return transcript_trace_g (TranscriptLevel::Warning, &[input_1], evaluator) .into_0 (),
		
		RuntimePrimitive1::TranscriptTraceNotice =>
			return transcript_trace_g (TranscriptLevel::Notice, &[input_1], evaluator) .into_0 (),
		
		RuntimePrimitive1::TranscriptTraceInformation =>
			return transcript_trace_g (TranscriptLevel::Information, &[input_1], evaluator) .into_0 (),
		
		RuntimePrimitive1::TranscriptTraceInternal =>
			return transcript_trace_g (TranscriptLevel::Internal, &[input_1], evaluator) .into_0 (),
		
		RuntimePrimitive1::TranscriptTraceDebugging =>
			return transcript_trace_g (TranscriptLevel::Debugging, &[input_1], evaluator) .into_0 (),
		
		RuntimePrimitive1::ProcessArgument =>
			return process_argument (input_1, evaluator),
		
		RuntimePrimitive1::ProcessEnvironmentVariable =>
			return process_environment_variable (input_1, evaluator),
		
		RuntimePrimitive1::ProcessExit =>
			return Err (try! (error_exit (Some (input_1), false)) .into ()),
		
		RuntimePrimitive1::ProcessExitEmergency =>
			return Err (try! (error_exit (Some (input_1), true)) .into ()),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitive1::ProcessSpawnExtended =>
			return process_spawn_extended (input_1, None, None, &mut Some (evaluator)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitive1::ProcessWaitPoll =>
			return process_wait (input_1, false) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitive1::ProcessWaitTry =>
			return process_wait (input_1, true) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitive1::ProcessWaitCheck =>
			return process_wait_check (input_1, true) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitive1::ProcessStdinGet =>
			return process_stdin_get (input_1),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitive1::ProcessStdoutGet =>
			return process_stdout_get (input_1),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitive1::ProcessStderrGet =>
			return process_stderr_get (input_1),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_2_evaluate (primitive : RuntimePrimitive2, input_1 : &Value, input_2 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		RuntimePrimitive2::ErrorRaise =>
			return Err (try! (error_build_1 (None, input_1, input_2))),
		
		RuntimePrimitive2::ErrorBuild =>
			return error_build_1 (None, input_1, input_2) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitive2::ParameterBuild =>
			return parameter_build (None, Some (input_1), Some (input_2), None, evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitive2::ParameterResolve =>
			return parameter_resolve (input_1, Some (input_2), evaluator),
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitive2::ParameterConfigure =>
			return parameter_configure (input_1, input_2, evaluator) .into_0 (),
		
		RuntimePrimitive2::TranscriptTraceCritical =>
			return transcript_trace_g (TranscriptLevel::Critical, &[input_1, input_2], evaluator) .into_0 (),
		
		RuntimePrimitive2::TranscriptTraceError =>
			return transcript_trace_g (TranscriptLevel::Error, &[input_1, input_2], evaluator) .into_0 (),
		
		RuntimePrimitive2::TranscriptTraceWarning =>
			return transcript_trace_g (TranscriptLevel::Warning, &[input_1, input_2], evaluator) .into_0 (),
		
		RuntimePrimitive2::TranscriptTraceNotice =>
			return transcript_trace_g (TranscriptLevel::Notice, &[input_1, input_2], evaluator) .into_0 (),
		
		RuntimePrimitive2::TranscriptTraceInformation =>
			return transcript_trace_g (TranscriptLevel::Information, &[input_1, input_2], evaluator) .into_0 (),
		
		RuntimePrimitive2::TranscriptTraceInternal =>
			return transcript_trace_g (TranscriptLevel::Internal, &[input_1, input_2], evaluator) .into_0 (),
		
		RuntimePrimitive2::TranscriptTraceDebugging =>
			return transcript_trace_g (TranscriptLevel::Debugging, &[input_1, input_2], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitive2::ProcessSpawnExtended =>
			return process_spawn_extended (input_1, Some (input_2), None, &mut Some (evaluator)) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_3_evaluate (primitive : RuntimePrimitive3, input_1 : &Value, input_2 : &Value, input_3 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		RuntimePrimitive3::ErrorRaise =>
			return Err (try! (error_build_2 (None, input_1, input_2, input_3))),
		
		RuntimePrimitive3::ErrorBuild =>
			return error_build_2 (None, input_1, input_2, input_3) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitive3::ParameterBuild =>
			return parameter_build (None, Some (input_1), Some (input_2), Some (try_as_boolean_ref! (input_3) .value ()), evaluator) .into_0 (),
		
		RuntimePrimitive3::TranscriptTraceCritical =>
			return transcript_trace_g (TranscriptLevel::Critical, &[input_1, input_2, input_3], evaluator) .into_0 (),
		
		RuntimePrimitive3::TranscriptTraceError =>
			return transcript_trace_g (TranscriptLevel::Error, &[input_1, input_2, input_3], evaluator) .into_0 (),
		
		RuntimePrimitive3::TranscriptTraceWarning =>
			return transcript_trace_g (TranscriptLevel::Warning, &[input_1, input_2, input_3], evaluator) .into_0 (),
		
		RuntimePrimitive3::TranscriptTraceNotice =>
			return transcript_trace_g (TranscriptLevel::Notice, &[input_1, input_2, input_3], evaluator) .into_0 (),
		
		RuntimePrimitive3::TranscriptTraceInformation =>
			return transcript_trace_g (TranscriptLevel::Information, &[input_1, input_2, input_3], evaluator) .into_0 (),
		
		RuntimePrimitive3::TranscriptTraceInternal =>
			return transcript_trace_g (TranscriptLevel::Internal, &[input_1, input_2, input_3], evaluator) .into_0 (),
		
		RuntimePrimitive3::TranscriptTraceDebugging =>
			return transcript_trace_g (TranscriptLevel::Debugging, &[input_1, input_2, input_3], evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitive3::ProcessSpawnExtended =>
			return process_spawn_extended (input_1, Some (input_2), Some (input_3), &mut Some (evaluator)) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_4_evaluate (primitive : RuntimePrimitive4, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		RuntimePrimitive4::ErrorRaise =>
			return Err (try! (error_build_3 (None, input_1, input_2, input_3, input_4))),
		
		RuntimePrimitive4::ErrorBuild =>
			return error_build_3 (None, input_1, input_2, input_3, input_4) .into_0 (),
		
		RuntimePrimitive4::TranscriptTraceCritical =>
			return transcript_trace_g (TranscriptLevel::Critical, &[input_1, input_2, input_3, input_4], evaluator) .into_0 (),
		
		RuntimePrimitive4::TranscriptTraceError =>
			return transcript_trace_g (TranscriptLevel::Error, &[input_1, input_2, input_3, input_4], evaluator) .into_0 (),
		
		RuntimePrimitive4::TranscriptTraceWarning =>
			return transcript_trace_g (TranscriptLevel::Warning, &[input_1, input_2, input_3, input_4], evaluator) .into_0 (),
		
		RuntimePrimitive4::TranscriptTraceNotice =>
			return transcript_trace_g (TranscriptLevel::Notice, &[input_1, input_2, input_3, input_4], evaluator) .into_0 (),
		
		RuntimePrimitive4::TranscriptTraceInformation =>
			return transcript_trace_g (TranscriptLevel::Information, &[input_1, input_2, input_3, input_4], evaluator) .into_0 (),
		
		RuntimePrimitive4::TranscriptTraceInternal =>
			return transcript_trace_g (TranscriptLevel::Internal, &[input_1, input_2, input_3, input_4], evaluator) .into_0 (),
		
		RuntimePrimitive4::TranscriptTraceDebugging =>
			return transcript_trace_g (TranscriptLevel::Debugging, &[input_1, input_2, input_3, input_4], evaluator) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_5_evaluate (primitive : RuntimePrimitive5, input_1 : &Value, input_2 : &Value, input_3 : &Value, input_4 : &Value, input_5 : &Value, evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		RuntimePrimitive5::ErrorRaise =>
			return Err (try! (error_build_4 (None, input_1, input_2, input_3, input_4, input_5))),
		
		RuntimePrimitive5::ErrorBuild =>
			return error_build_4 (None, input_1, input_2, input_3, input_4, input_5) .into_0 (),
		
		RuntimePrimitive5::TranscriptTraceCritical =>
			return transcript_trace_g (TranscriptLevel::Critical, &[input_1, input_2, input_3, input_4, input_5], evaluator) .into_0 (),
		
		RuntimePrimitive5::TranscriptTraceError =>
			return transcript_trace_g (TranscriptLevel::Error, &[input_1, input_2, input_3, input_4, input_5], evaluator) .into_0 (),
		
		RuntimePrimitive5::TranscriptTraceWarning =>
			return transcript_trace_g (TranscriptLevel::Warning, &[input_1, input_2, input_3, input_4, input_5], evaluator) .into_0 (),
		
		RuntimePrimitive5::TranscriptTraceNotice =>
			return transcript_trace_g (TranscriptLevel::Notice, &[input_1, input_2, input_3, input_4, input_5], evaluator) .into_0 (),
		
		RuntimePrimitive5::TranscriptTraceInformation =>
			return transcript_trace_g (TranscriptLevel::Information, &[input_1, input_2, input_3, input_4, input_5], evaluator) .into_0 (),
		
		RuntimePrimitive5::TranscriptTraceInternal =>
			return transcript_trace_g (TranscriptLevel::Internal, &[input_1, input_2, input_3, input_4, input_5], evaluator) .into_0 (),
		
		RuntimePrimitive5::TranscriptTraceDebugging =>
			return transcript_trace_g (TranscriptLevel::Debugging, &[input_1, input_2, input_3, input_4, input_5], evaluator) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_n_evaluate (primitive : RuntimePrimitiveN, inputs : &[&Value], evaluator : &mut EvaluatorContext) -> (Outcome<Value>) {
	match primitive {
		
		RuntimePrimitiveN::ErrorRaise => {
			let (message, inputs) = try_some! (inputs.split_first (), 0x84aec603);
			return Err (try! (error_build_n (None, message, inputs)));
		},
		
		RuntimePrimitiveN::ErrorBuild => {
			let (message, inputs) = try_some! (inputs.split_first (), 0x87db450f);
			return error_build_n (None, message, inputs) .into_0 ();
		},
		
		RuntimePrimitiveN::TranscriptTraceCritical =>
			return transcript_trace_g (TranscriptLevel::Critical, inputs, evaluator) .into_0 (),
		
		RuntimePrimitiveN::TranscriptTraceError =>
			return transcript_trace_g (TranscriptLevel::Error, inputs, evaluator) .into_0 (),
		
		RuntimePrimitiveN::TranscriptTraceWarning =>
			return transcript_trace_g (TranscriptLevel::Warning, inputs, evaluator) .into_0 (),
		
		RuntimePrimitiveN::TranscriptTraceNotice =>
			return transcript_trace_g (TranscriptLevel::Notice, inputs, evaluator) .into_0 (),
		
		RuntimePrimitiveN::TranscriptTraceInformation =>
			return transcript_trace_g (TranscriptLevel::Information, inputs, evaluator) .into_0 (),
		
		RuntimePrimitiveN::TranscriptTraceInternal =>
			return transcript_trace_g (TranscriptLevel::Internal, inputs, evaluator) .into_0 (),
		
		RuntimePrimitiveN::TranscriptTraceDebugging =>
			return transcript_trace_g (TranscriptLevel::Debugging, inputs, evaluator) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveN::ProcessSpawn =>
			return process_spawn (inputs, &mut Some (evaluator)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveN::ProcessRunTry =>
			return process_run (inputs, &mut Some (evaluator)) .into_0 (),
		
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveN::ProcessRunCheck =>
			return process_run_check (inputs, &mut Some (evaluator)) .into_0 (),
		
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_v_alternative_0 (primitive : RuntimePrimitiveV) -> (Option<RuntimePrimitive0>) {
	match primitive {
		RuntimePrimitiveV::ErrorRaise =>
			None,
		RuntimePrimitiveV::ErrorBuild =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterBuild =>
			Some (RuntimePrimitive0::ParameterBuild),
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterResolve =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterConfigure =>
			None,
		RuntimePrimitiveV::TranscriptTraceCritical =>
			None,
		RuntimePrimitiveV::TranscriptTraceError =>
			None,
		RuntimePrimitiveV::TranscriptTraceWarning =>
			None,
		RuntimePrimitiveV::TranscriptTraceNotice =>
			None,
		RuntimePrimitiveV::TranscriptTraceInformation =>
			None,
		RuntimePrimitiveV::TranscriptTraceInternal =>
			None,
		RuntimePrimitiveV::TranscriptTraceDebugging =>
			None,
		RuntimePrimitiveV::ProcessExit =>
			Some (RuntimePrimitive0::ProcessExit),
		RuntimePrimitiveV::ProcessExitEmergency =>
			Some (RuntimePrimitive0::ProcessExitEmergency),
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveV::ProcessSpawnExtended =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_v_alternative_1 (primitive : RuntimePrimitiveV) -> (Option<RuntimePrimitive1>) {
	match primitive {
		RuntimePrimitiveV::ErrorRaise =>
			Some (RuntimePrimitive1::ErrorRaise),
		RuntimePrimitiveV::ErrorBuild =>
			Some (RuntimePrimitive1::ErrorBuild),
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterBuild =>
			Some (RuntimePrimitive1::ParameterBuild),
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterResolve =>
			Some (RuntimePrimitive1::ParameterResolve),
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterConfigure =>
			None,
		RuntimePrimitiveV::TranscriptTraceCritical =>
			Some (RuntimePrimitive1::TranscriptTraceCritical),
		RuntimePrimitiveV::TranscriptTraceError =>
			Some (RuntimePrimitive1::TranscriptTraceError),
		RuntimePrimitiveV::TranscriptTraceWarning =>
			Some (RuntimePrimitive1::TranscriptTraceWarning),
		RuntimePrimitiveV::TranscriptTraceNotice =>
			Some (RuntimePrimitive1::TranscriptTraceNotice),
		RuntimePrimitiveV::TranscriptTraceInformation =>
			Some (RuntimePrimitive1::TranscriptTraceInformation),
		RuntimePrimitiveV::TranscriptTraceInternal =>
			Some (RuntimePrimitive1::TranscriptTraceInternal),
		RuntimePrimitiveV::TranscriptTraceDebugging =>
			Some (RuntimePrimitive1::TranscriptTraceDebugging),
		RuntimePrimitiveV::ProcessExit =>
			Some (RuntimePrimitive1::ProcessExit),
		RuntimePrimitiveV::ProcessExitEmergency =>
			Some (RuntimePrimitive1::ProcessExitEmergency),
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveV::ProcessSpawnExtended =>
			Some (RuntimePrimitive1::ProcessSpawnExtended),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_v_alternative_2 (primitive : RuntimePrimitiveV) -> (Option<RuntimePrimitive2>) {
	match primitive {
		RuntimePrimitiveV::ErrorRaise =>
			Some (RuntimePrimitive2::ErrorRaise),
		RuntimePrimitiveV::ErrorBuild =>
			Some (RuntimePrimitive2::ErrorBuild),
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterBuild =>
			Some (RuntimePrimitive2::ParameterBuild),
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterResolve =>
			Some (RuntimePrimitive2::ParameterResolve),
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterConfigure =>
			Some (RuntimePrimitive2::ParameterConfigure),
		RuntimePrimitiveV::TranscriptTraceCritical =>
			Some (RuntimePrimitive2::TranscriptTraceCritical),
		RuntimePrimitiveV::TranscriptTraceError =>
			Some (RuntimePrimitive2::TranscriptTraceError),
		RuntimePrimitiveV::TranscriptTraceWarning =>
			Some (RuntimePrimitive2::TranscriptTraceWarning),
		RuntimePrimitiveV::TranscriptTraceNotice =>
			Some (RuntimePrimitive2::TranscriptTraceNotice),
		RuntimePrimitiveV::TranscriptTraceInformation =>
			Some (RuntimePrimitive2::TranscriptTraceInformation),
		RuntimePrimitiveV::TranscriptTraceInternal =>
			Some (RuntimePrimitive2::TranscriptTraceInternal),
		RuntimePrimitiveV::TranscriptTraceDebugging =>
			Some (RuntimePrimitive2::TranscriptTraceDebugging),
		RuntimePrimitiveV::ProcessExit =>
			None,
		RuntimePrimitiveV::ProcessExitEmergency =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveV::ProcessSpawnExtended =>
			Some (RuntimePrimitive2::ProcessSpawnExtended),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_v_alternative_3 (primitive : RuntimePrimitiveV) -> (Option<RuntimePrimitive3>) {
	match primitive {
		RuntimePrimitiveV::ErrorRaise =>
			Some (RuntimePrimitive3::ErrorRaise),
		RuntimePrimitiveV::ErrorBuild =>
			Some (RuntimePrimitive3::ErrorBuild),
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterBuild =>
			Some (RuntimePrimitive3::ParameterBuild),
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterResolve =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterConfigure =>
			None,
		RuntimePrimitiveV::TranscriptTraceCritical =>
			Some (RuntimePrimitive3::TranscriptTraceCritical),
		RuntimePrimitiveV::TranscriptTraceError =>
			Some (RuntimePrimitive3::TranscriptTraceError),
		RuntimePrimitiveV::TranscriptTraceWarning =>
			Some (RuntimePrimitive3::TranscriptTraceWarning),
		RuntimePrimitiveV::TranscriptTraceNotice =>
			Some (RuntimePrimitive3::TranscriptTraceNotice),
		RuntimePrimitiveV::TranscriptTraceInformation =>
			Some (RuntimePrimitive3::TranscriptTraceInformation),
		RuntimePrimitiveV::TranscriptTraceInternal =>
			Some (RuntimePrimitive3::TranscriptTraceInternal),
		RuntimePrimitiveV::TranscriptTraceDebugging =>
			Some (RuntimePrimitive3::TranscriptTraceDebugging),
		RuntimePrimitiveV::ProcessExit =>
			None,
		RuntimePrimitiveV::ProcessExitEmergency =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveV::ProcessSpawnExtended =>
			Some (RuntimePrimitive3::ProcessSpawnExtended),
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_v_alternative_4 (primitive : RuntimePrimitiveV) -> (Option<RuntimePrimitive4>) {
	match primitive {
		RuntimePrimitiveV::ErrorRaise =>
			Some (RuntimePrimitive4::ErrorRaise),
		RuntimePrimitiveV::ErrorBuild =>
			Some (RuntimePrimitive4::ErrorBuild),
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterBuild =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterResolve =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterConfigure =>
			None,
		RuntimePrimitiveV::TranscriptTraceCritical =>
			Some (RuntimePrimitive4::TranscriptTraceCritical),
		RuntimePrimitiveV::TranscriptTraceError =>
			Some (RuntimePrimitive4::TranscriptTraceError),
		RuntimePrimitiveV::TranscriptTraceWarning =>
			Some (RuntimePrimitive4::TranscriptTraceWarning),
		RuntimePrimitiveV::TranscriptTraceNotice =>
			Some (RuntimePrimitive4::TranscriptTraceNotice),
		RuntimePrimitiveV::TranscriptTraceInformation =>
			Some (RuntimePrimitive4::TranscriptTraceInformation),
		RuntimePrimitiveV::TranscriptTraceInternal =>
			Some (RuntimePrimitive4::TranscriptTraceInternal),
		RuntimePrimitiveV::TranscriptTraceDebugging =>
			Some (RuntimePrimitive4::TranscriptTraceDebugging),
		RuntimePrimitiveV::ProcessExit =>
			None,
		RuntimePrimitiveV::ProcessExitEmergency =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveV::ProcessSpawnExtended =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_v_alternative_5 (primitive : RuntimePrimitiveV) -> (Option<RuntimePrimitive5>) {
	match primitive {
		RuntimePrimitiveV::ErrorRaise =>
			Some (RuntimePrimitive5::ErrorRaise),
		RuntimePrimitiveV::ErrorBuild =>
			Some (RuntimePrimitive5::ErrorBuild),
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterBuild =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterResolve =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterConfigure =>
			None,
		RuntimePrimitiveV::TranscriptTraceCritical =>
			Some (RuntimePrimitive5::TranscriptTraceCritical),
		RuntimePrimitiveV::TranscriptTraceError =>
			Some (RuntimePrimitive5::TranscriptTraceError),
		RuntimePrimitiveV::TranscriptTraceWarning =>
			Some (RuntimePrimitive5::TranscriptTraceWarning),
		RuntimePrimitiveV::TranscriptTraceNotice =>
			Some (RuntimePrimitive5::TranscriptTraceNotice),
		RuntimePrimitiveV::TranscriptTraceInformation =>
			Some (RuntimePrimitive5::TranscriptTraceInformation),
		RuntimePrimitiveV::TranscriptTraceInternal =>
			Some (RuntimePrimitive5::TranscriptTraceInternal),
		RuntimePrimitiveV::TranscriptTraceDebugging =>
			Some (RuntimePrimitive5::TranscriptTraceDebugging),
		RuntimePrimitiveV::ProcessExit =>
			None,
		RuntimePrimitiveV::ProcessExitEmergency =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveV::ProcessSpawnExtended =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_v_alternative_n (primitive : RuntimePrimitiveV) -> (Option<RuntimePrimitiveN>) {
	match primitive {
		RuntimePrimitiveV::ErrorRaise =>
			Some (RuntimePrimitiveN::ErrorRaise),
		RuntimePrimitiveV::ErrorBuild =>
			Some (RuntimePrimitiveN::ErrorBuild),
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterBuild =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterResolve =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_parameters" ) ]
		RuntimePrimitiveV::ParameterConfigure =>
			None,
		RuntimePrimitiveV::TranscriptTraceCritical =>
			Some (RuntimePrimitiveN::TranscriptTraceCritical),
		RuntimePrimitiveV::TranscriptTraceError =>
			Some (RuntimePrimitiveN::TranscriptTraceError),
		RuntimePrimitiveV::TranscriptTraceWarning =>
			Some (RuntimePrimitiveN::TranscriptTraceWarning),
		RuntimePrimitiveV::TranscriptTraceNotice =>
			Some (RuntimePrimitiveN::TranscriptTraceNotice),
		RuntimePrimitiveV::TranscriptTraceInformation =>
			Some (RuntimePrimitiveN::TranscriptTraceInformation),
		RuntimePrimitiveV::TranscriptTraceInternal =>
			Some (RuntimePrimitiveN::TranscriptTraceInternal),
		RuntimePrimitiveV::TranscriptTraceDebugging =>
			Some (RuntimePrimitiveN::TranscriptTraceDebugging),
		RuntimePrimitiveV::ProcessExit =>
			None,
		RuntimePrimitiveV::ProcessExitEmergency =>
			None,
		#[ cfg ( feature = "vonuvoli_builtins_processes" ) ]
		RuntimePrimitiveV::ProcessSpawnExtended =>
			None,
	}
}




#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_0_attributes (_primitive : RuntimePrimitive0) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_1_attributes (_primitive : RuntimePrimitive1) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_2_attributes (_primitive : RuntimePrimitive2) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_3_attributes (_primitive : RuntimePrimitive3) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_4_attributes (_primitive : RuntimePrimitive4) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_5_attributes (_primitive : RuntimePrimitive5) -> (Option<ProcedureAttributes>) {
	return None;
}

#[ cfg_attr ( feature = "vonuvoli_inline", inline ) ]
pub fn runtime_primitive_n_attributes (_primitive : RuntimePrimitiveN) -> (Option<ProcedureAttributes>) {
	return None;
}

