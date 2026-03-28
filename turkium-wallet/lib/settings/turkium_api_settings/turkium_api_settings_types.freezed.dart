// GENERATED CODE - DO NOT MODIFY BY HAND
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'turkium_api_settings_types.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

// dart format off
T _$identity<T>(T value) => value;

/// @nodoc
mixin _$TurkiumApiSettings {

 Map<String, String> get apiUrlByNetworkId;
/// Create a copy of TurkiumApiSettings
/// with the given fields replaced by the non-null parameter values.
@JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
$TurkiumApiSettingsCopyWith<TurkiumApiSettings> get copyWith => _$TurkiumApiSettingsCopyWithImpl<TurkiumApiSettings>(this as TurkiumApiSettings, _$identity);

  /// Serializes this TurkiumApiSettings to a JSON map.
  Map<String, dynamic> toJson();


@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is TurkiumApiSettings&&const DeepCollectionEquality().equals(other.apiUrlByNetworkId, apiUrlByNetworkId));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,const DeepCollectionEquality().hash(apiUrlByNetworkId));

@override
String toString() {
  return 'TurkiumApiSettings(apiUrlByNetworkId: $apiUrlByNetworkId)';
}


}

/// @nodoc
abstract mixin class $TurkiumApiSettingsCopyWith<$Res>  {
  factory $TurkiumApiSettingsCopyWith(TurkiumApiSettings value, $Res Function(TurkiumApiSettings) _then) = _$TurkiumApiSettingsCopyWithImpl;
@useResult
$Res call({
 Map<String, String> apiUrlByNetworkId
});




}
/// @nodoc
class _$TurkiumApiSettingsCopyWithImpl<$Res>
    implements $TurkiumApiSettingsCopyWith<$Res> {
  _$TurkiumApiSettingsCopyWithImpl(this._self, this._then);

  final TurkiumApiSettings _self;
  final $Res Function(TurkiumApiSettings) _then;

/// Create a copy of TurkiumApiSettings
/// with the given fields replaced by the non-null parameter values.
@pragma('vm:prefer-inline') @override $Res call({Object? apiUrlByNetworkId = null,}) {
  return _then(_self.copyWith(
apiUrlByNetworkId: null == apiUrlByNetworkId ? _self.apiUrlByNetworkId : apiUrlByNetworkId // ignore: cast_nullable_to_non_nullable
as Map<String, String>,
  ));
}

}


/// Adds pattern-matching-related methods to [TurkiumApiSettings].
extension TurkiumApiSettingsPatterns on TurkiumApiSettings {
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

@optionalTypeArgs TResult maybeMap<TResult extends Object?>(TResult Function( _TurkiumApiSettings value)?  $default,{required TResult orElse(),}){
final _that = this;
switch (_that) {
case _TurkiumApiSettings() when $default != null:
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

@optionalTypeArgs TResult map<TResult extends Object?>(TResult Function( _TurkiumApiSettings value)  $default,){
final _that = this;
switch (_that) {
case _TurkiumApiSettings():
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

@optionalTypeArgs TResult? mapOrNull<TResult extends Object?>(TResult? Function( _TurkiumApiSettings value)?  $default,){
final _that = this;
switch (_that) {
case _TurkiumApiSettings() when $default != null:
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

@optionalTypeArgs TResult maybeWhen<TResult extends Object?>(TResult Function( Map<String, String> apiUrlByNetworkId)?  $default,{required TResult orElse(),}) {final _that = this;
switch (_that) {
case _TurkiumApiSettings() when $default != null:
return $default(_that.apiUrlByNetworkId);case _:
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

@optionalTypeArgs TResult when<TResult extends Object?>(TResult Function( Map<String, String> apiUrlByNetworkId)  $default,) {final _that = this;
switch (_that) {
case _TurkiumApiSettings():
return $default(_that.apiUrlByNetworkId);}
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

@optionalTypeArgs TResult? whenOrNull<TResult extends Object?>(TResult? Function( Map<String, String> apiUrlByNetworkId)?  $default,) {final _that = this;
switch (_that) {
case _TurkiumApiSettings() when $default != null:
return $default(_that.apiUrlByNetworkId);case _:
  return null;

}
}

}

/// @nodoc
@JsonSerializable()

class _TurkiumApiSettings extends TurkiumApiSettings {
  const _TurkiumApiSettings({final  Map<String, String> apiUrlByNetworkId = const {}}): _apiUrlByNetworkId = apiUrlByNetworkId,super._();
  factory _TurkiumApiSettings.fromJson(Map<String, dynamic> json) => _$TurkiumApiSettingsFromJson(json);

 final  Map<String, String> _apiUrlByNetworkId;
@override@JsonKey() Map<String, String> get apiUrlByNetworkId {
  if (_apiUrlByNetworkId is EqualUnmodifiableMapView) return _apiUrlByNetworkId;
  // ignore: implicit_dynamic_type
  return EqualUnmodifiableMapView(_apiUrlByNetworkId);
}


/// Create a copy of TurkiumApiSettings
/// with the given fields replaced by the non-null parameter values.
@override @JsonKey(includeFromJson: false, includeToJson: false)
@pragma('vm:prefer-inline')
_$TurkiumApiSettingsCopyWith<_TurkiumApiSettings> get copyWith => __$TurkiumApiSettingsCopyWithImpl<_TurkiumApiSettings>(this, _$identity);

@override
Map<String, dynamic> toJson() {
  return _$TurkiumApiSettingsToJson(this, );
}

@override
bool operator ==(Object other) {
  return identical(this, other) || (other.runtimeType == runtimeType&&other is _TurkiumApiSettings&&const DeepCollectionEquality().equals(other._apiUrlByNetworkId, _apiUrlByNetworkId));
}

@JsonKey(includeFromJson: false, includeToJson: false)
@override
int get hashCode => Object.hash(runtimeType,const DeepCollectionEquality().hash(_apiUrlByNetworkId));

@override
String toString() {
  return 'TurkiumApiSettings(apiUrlByNetworkId: $apiUrlByNetworkId)';
}


}

/// @nodoc
abstract mixin class _$TurkiumApiSettingsCopyWith<$Res> implements $TurkiumApiSettingsCopyWith<$Res> {
  factory _$TurkiumApiSettingsCopyWith(_TurkiumApiSettings value, $Res Function(_TurkiumApiSettings) _then) = __$TurkiumApiSettingsCopyWithImpl;
@override @useResult
$Res call({
 Map<String, String> apiUrlByNetworkId
});




}
/// @nodoc
class __$TurkiumApiSettingsCopyWithImpl<$Res>
    implements _$TurkiumApiSettingsCopyWith<$Res> {
  __$TurkiumApiSettingsCopyWithImpl(this._self, this._then);

  final _TurkiumApiSettings _self;
  final $Res Function(_TurkiumApiSettings) _then;

/// Create a copy of TurkiumApiSettings
/// with the given fields replaced by the non-null parameter values.
@override @pragma('vm:prefer-inline') $Res call({Object? apiUrlByNetworkId = null,}) {
  return _then(_TurkiumApiSettings(
apiUrlByNetworkId: null == apiUrlByNetworkId ? _self._apiUrlByNetworkId : apiUrlByNetworkId // ignore: cast_nullable_to_non_nullable
as Map<String, String>,
  ));
}


}

// dart format on
