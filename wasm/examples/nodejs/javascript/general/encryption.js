const Turkium = require('../../../../nodejs/Turkium');

Turkium.initConsolePanicHook();

(async () => {

    let encrypted = Turkium.encryptXChaCha20Poly1305("my message", "my_password");
    console.log("encrypted:", encrypted);
    let decrypted = Turkium.decryptXChaCha20Poly1305(encrypted, "my_password");
    console.log("decrypted:", decrypted);

})();
