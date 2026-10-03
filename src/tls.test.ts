import { describe, it, expect } from 'vite-plus/test';
import { TLS, TlsProvider } from '../index.js';

describe('TLS module', () => {
  it('default TLS provider is ring', () => {
    const tls = new TLS();
    expect(tls.getProviderName()).toBe('ring');
    expect(tls.isSupported()).toBe(true);
  });

  it('explicit Openssl provider', () => {
    const tls = new TLS(TlsProvider.Openssl);
    expect(tls.getProviderName()).toBe('openssl');
    expect(tls.isSupported()).toBe(true);
  });

  it('explicit BoringSSL provider', () => {
    const tls = new TLS(TlsProvider.Btls);
    expect(tls.getProviderName()).toBe('boringssl');
    expect(tls.isSupported()).toBe(true);
  });

  it('explicit Mbedtls provider', () => {
    const tls = new TLS(TlsProvider.Mbedtls);
    expect(tls.getProviderName()).toBe('mbedtls');
    expect(tls.isSupported()).toBe(false);
  });
});
