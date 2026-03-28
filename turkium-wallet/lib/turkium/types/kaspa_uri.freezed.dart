// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'kaspa_uri.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;
/// @nodoc
mixin _$TurkiumUriParam {

 String get key; String get value;
/// Create a copy of TurkiumUriParam
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$TurkiumUriParamCopyWith<TurkiumUriParam> get copyWith => _$TurkiumUriParamCopyWithImpl<TurkiumUriParam>(this as TurkiumUriParam, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is TurkiumUriParam&&(identical(other.key, key) || other.key == key)&&(identical(other.value, value) || other.value == value));
}


@override
int get hashCode => Object.hash(runtimeType,key,value);

@override
String toString() {
  return 'TurkiumUriParam(key: $key, value: $value)';
}


}

/// @nodoc
abstract mixin class $TurkiumUriParamCopyWith<$Res>  {
  factory $TurkiumUriParamCopyWith(TurkiumUriParam value, $Res Function(TurkiumUriParam) _then) = _$TurkiumUriParamCopyWithImpl;
@useResult
$Res call({
 String key, String value
});




}
/// @nodoc
class _$TurkiumUriParamCopyWithImpl<$Res>
    implements $TurkiumUriParamCopyWith<$Res> {
  _$TurkiumUriParamCopyWithImpl(this._self, this._then);

  final TurkiumUriParam _self;
  final $Res Function(TurkiumUriParam) _then;

/// Create a copy of TurkiumUriParam
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? key = null,Object? value = null,}) {
  return _then(_self.copyWith(
key: null == key ? _self.key : key // ignore: cast_nullable_to_non_nullable
as String,value: null == value ? _self.value : value // ignore: cast_nullable_to_non_nullable
as String,
  ));
}

}


/// Adds pattern-matching-related methods to [TurkiumUriParam].
extension TurkiumUriParamPatterns on TurkiumUriParam {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _TurkiumUriParam value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _TurkiumUriParam() when $default != null:
return $default(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _TurkiumUriParam value)  $default,){
final _that = this;
switch (_that) {
case _TurkiumUriParam():
return $default(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _TurkiumUriParam value)?  $default,){
final _that = this;
switch (_that) {
case _TurkiumUriParam() when $default != null:
return $default(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( String key,  String value)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _TurkiumUriParam() when $default != null:
return $default(_that.key,_that.value);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( String key,  String value)  $default,) {final _that = this;
switch (_that) {
case _TurkiumUriParam():
return $default(_that.key,_that.value);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( String key,  String value)?  $default,) {final _that = this;
switch (_that) {
case _TurkiumUriParam() when $default != null:
return $default(_that.key,_that.value);case _:
  return null;

}
}

}

/// @nodoc


class _TurkiumUriParam extends TurkiumUriParam {
  const _TurkiumUriParam({required this.key, required this.value}): super._();
  

@override final  String key;
@override final  String value;

/// Create a copy of TurkiumUriParam
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$TurkiumUriParamCopyWith<_TurkiumUriParam> get copyWith => __$TurkiumUriParamCopyWithImpl<_TurkiumUriParam>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _TurkiumUriParam&&(identical(other.key, key) || other.key == key)&&(identical(other.value, value) || other.value == value));
}


@override
int get hashCode => Object.hash(runtimeType,key,value);

@override
String toString() {
  return 'TurkiumUriParam(key: $key, value: $value)';
}


}

/// @nodoc
abstract mixin class _$TurkiumUriParamCopyWith<$Res> implements $TurkiumUriParamCopyWith<$Res> {
  factory _$TurkiumUriParamCopyWith(_TurkiumUriParam value, $Res Function(_TurkiumUriParam) _then) = __$TurkiumUriParamCopyWithImpl;
@override @useResult
$Res call({
 String key, String value
});




}
/// @nodoc
class __$TurkiumUriParamCopyWithImpl<$Res>
    implements _$TurkiumUriParamCopyWith<$Res> {
  __$TurkiumUriParamCopyWithImpl(this._self, this._then);

  final _TurkiumUriParam _self;
  final $Res Function(_TurkiumUriParam) _then;

/// Create a copy of TurkiumUriParam
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? key = null,Object? value = null,}) {
  return _then(_TurkiumUriParam(
key: null == key ? _self.key : key // ignore: cast_nullable_to_non_nullable
as String,value: null == value ? _self.value : value // ignore: cast_nullable_to_non_nullable
as String,
  ));
}


}

/// @nodoc
mixin _$TurkiumUri {

 Address get address; Amount? get amount; String? get label; String? get message; IList<TurkiumUriParam> get others;
/// Create a copy of TurkiumUri
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$TurkiumUriCopyWith<TurkiumUri> get copyWith => _$TurkiumUriCopyWithImpl<TurkiumUri>(this as TurkiumUri, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is TurkiumUri&&(identical(other.address, address) || other.address == address)&&(identical(other.amount, amount) || other.amount == amount)&&(identical(other.label, label) || other.label == label)&&(identical(other.message, message) || other.message == message)&&const DeepCollectionEquality().equals(other.others, others));
}


@override
int get hashCode => Object.hash(runtimeType,address,amount,label,message,const DeepCollectionEquality().hash(others));



}

/// @nodoc
abstract mixin class $TurkiumUriCopyWith<$Res>  {
  factory $TurkiumUriCopyWith(TurkiumUri value, $Res Function(TurkiumUri) _then) = _$TurkiumUriCopyWithImpl;
@useResult
$Res call({
 Address address, Amount? amount, String? label, String? message, IList<TurkiumUriParam> others
});


$AddressCopyWith<$Res> get address;$AmountCopyWith<$Res>? get amount;

}
/// @nodoc
class _$TurkiumUriCopyWithImpl<$Res>
    implements $TurkiumUriCopyWith<$Res> {
  _$TurkiumUriCopyWithImpl(this._self, this._then);

  final TurkiumUri _self;
  final $Res Function(TurkiumUri) _then;

/// Create a copy of TurkiumUri
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? address = null,Object? amount = freezed,Object? label = freezed,Object? message = freezed,Object? others = null,}) {
  return _then(_self.copyWith(
address: null == address ? _self.address : address // ignore: cast_nullable_to_non_nullable
as Address,amount: freezed == amount ? _self.amount : amount // ignore: cast_nullable_to_non_nullable
as Amount?,label: freezed == label ? _self.label : label // ignore: cast_nullable_to_non_nullable
as String?,message: freezed == message ? _self.message : message // ignore: cast_nullable_to_non_nullable
as String?,others: null == others ? _self.others : others // ignore: cast_nullable_to_non_nullable
as IList<TurkiumUriParam>,
  ));
}
/// Create a copy of TurkiumUri
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$AddressCopyWith<$Res> get address {
  
  return $AddressCopyWith<$Res>(_self.address, (value) {
    return _then(_self.copyWith(address: value));
  });
}/// Create a copy of TurkiumUri
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$AmountCopyWith<$Res>? get amount {
    if (_self.amount == null) {
    return null;
  }

  return $AmountCopyWith<$Res>(_self.amount!, (value) {
    return _then(_self.copyWith(amount: value));
  });
}
}


/// Adds pattern-matching-related methods to [TurkiumUri].
extension TurkiumUriPatterns on TurkiumUri {
/// A variant of `map` that fallback to returning `orElse`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _TurkiumUri value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _TurkiumUri() when $default != null:
return $default(_that);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// Callbacks receives the raw object, upcasted.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case final Subclass2 value:
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _TurkiumUri value)  $default,){
final _that = this;
switch (_that) {
case _TurkiumUri():
return $default(_that);}
}
/// A variant of `map` that fallback to returning `null`.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case final Subclass value:
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _TurkiumUri value)?  $default,){
final _that = this;
switch (_that) {
case _TurkiumUri() when $default != null:
return $default(_that);case _:
  return null;

}
}
/// A variant of `when` that fallback to an `orElse` callback.
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return orElse();
/// }
/// ```

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( Address address,  Amount? amount,  String? label,  String? message,  IList<TurkiumUriParam> others)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _TurkiumUri() when $default != null:
return $default(_that.address,_that.amount,_that.label,_that.message,_that.others);case _:
  return orElse();

}
}
/// A `switch`-like method, using callbacks.
///
/// As opposed to `map`, this offers destructuring.
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case Subclass2(:final field2):
///     return ...;
/// }
/// ```

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( Address address,  Amount? amount,  String? label,  String? message,  IList<TurkiumUriParam> others)  $default,) {final _that = this;
switch (_that) {
case _TurkiumUri():
return $default(_that.address,_that.amount,_that.label,_that.message,_that.others);}
}
/// A variant of `when` that fallback to returning `null`
///
/// It is equivalent to doing:
/// ```dart
/// switch (sealedClass) {
///   case Subclass(:final field):
///     return ...;
///   case _:
///     return null;
/// }
/// ```

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( Address address,  Amount? amount,  String? label,  String? message,  IList<TurkiumUriParam> others)?  $default,) {final _that = this;
switch (_that) {
case _TurkiumUri() when $default != null:
return $default(_that.address,_that.amount,_that.label,_that.message,_that.others);case _:
  return null;

}
}

}

/// @nodoc


class _TurkiumUri extends TurkiumUri {
  const _TurkiumUri({required this.address, this.amount, this.label, this.message, this.others = const IListConst([])}): super._();
  

@override final  Address address;
@override final  Amount? amount;
@override final  String? label;
@override final  String? message;
@override@JsonKey() final  IList<TurkiumUriParam> others;

/// Create a copy of TurkiumUri
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$TurkiumUriCopyWith<_TurkiumUri> get copyWith => __$TurkiumUriCopyWithImpl<_TurkiumUri>(this, _$identity);



@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _TurkiumUri&&(identical(other.address, address) || other.address == address)&&(identical(other.amount, amount) || other.amount == amount)&&(identical(other.label, label) || other.label == label)&&(identical(other.message, message) || other.message == message)&&const DeepCollectionEquality().equals(other.others, others));
}


@override
int get hashCode => Object.hash(runtimeType,address,amount,label,message,const DeepCollectionEquality().hash(others));



}

/// @nodoc
abstract mixin class _$TurkiumUriCopyWith<$Res> implements $TurkiumUriCopyWith<$Res> {
  factory _$TurkiumUriCopyWith(_TurkiumUri value, $Res Function(_TurkiumUri) _then) = __$TurkiumUriCopyWithImpl;
@override @useResult
$Res call({
 Address address, Amount? amount, String? label, String? message, IList<TurkiumUriParam> others
});


@override $AddressCopyWith<$Res> get address;@override $AmountCopyWith<$Res>? get amount;

}
/// @nodoc
class __$TurkiumUriCopyWithImpl<$Res>
    implements _$TurkiumUriCopyWith<$Res> {
  __$TurkiumUriCopyWithImpl(this._self, this._then);

  final _TurkiumUri _self;
  final $Res Function(_TurkiumUri) _then;

/// Create a copy of TurkiumUri
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? address = null,Object? amount = freezed,Object? label = freezed,Object? message = freezed,Object? others = null,}) {
  return _then(_TurkiumUri(
address: null == address ? _self.address : address // ignore: cast_nullable_to_non_nullable
as Address,amount: freezed == amount ? _self.amount : amount // ignore: cast_nullable_to_non_nullable
as Amount?,label: freezed == label ? _self.label : label // ignore: cast_nullable_to_non_nullable
as String?,message: freezed == message ? _self.message : message // ignore: cast_nullable_to_non_nullable
as String?,others: null == others ? _self.others : others // ignore: cast_nullable_to_non_nullable
as IList<TurkiumUriParam>,
  ));
}

/// Create a copy of TurkiumUri
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$AddressCopyWith<$Res> get address {
  
  return $AddressCopyWith<$Res>(_self.address, (value) {
    return _then(_self.copyWith(address: value));
  });
}/// Create a copy of TurkiumUri
/// with the given fields replaced by the non-null parameter values.
@override
@pragma('vm:prefer-inline')
$AmountCopyWith<$Res>? get amount {
    if (_self.amount == null) {
    return null;
  }

  return $AmountCopyWith<$Res>(_self.amount!, (value) {
    return _then(_self.copyWith(amount: value));
  });
}
}

// dart format on
