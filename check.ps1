cargo fmt --all
cargo clippy

$crates = @(
  "Turkium-wrpc-wasm",
  "Turkium-wallet-cli-wasm",
  "Turkium-wasm",
  "Turkium-cli",
  "Turkium-os",
  "Turkium-daemon"
)

$env:AR="llvm-ar"
foreach ($crate in $crates)
{
  Write-Output "`ncargo clippy -p $crate --target wasm32-unknown-unknown"
  cargo clippy -p $crate --target wasm32-unknown-unknown
  $status=$LASTEXITCODE
  if($status -ne 0) {
    Write-Output "`n--> wasm32 check of $crate failed`n"
    break
  }
}
$env:AR=""