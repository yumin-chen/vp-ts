import { test, expect } from 'vite-plus/test'
import { Tls, createTls, getDefaultProviderName, getSupportedProviders } from '../index.js'

test('TLS Module - create default TLS instance using ring provider', () => {
  const tls = new Tls()
  expect(tls.getProviderName()).toBe('ring')
  expect(tls.isSupported()).toBe(true)
})

test('TLS Module - create TLS with specified provider', () => {
  const tlsOpenssl = createTls('openssl')
  expect(tlsOpenssl.getProviderName()).toBe('openssl')

  const tlsBtls = new Tls('btls')
  expect(tlsBtls.getProviderName()).toBe('btls')

  const tlsMbed = new Tls('mbedtls')
  expect(tlsMbed.getProviderName()).toBe('mbedtls')
})

test('TLS Module - list supported providers and default provider name', () => {
  expect(getDefaultProviderName()).toBe('ring')
  const providers = getSupportedProviders()
  expect(providers).toContain('ring')
  expect(providers).toContain('openssl')
  expect(providers).toContain('btls')
  expect(providers).toContain('mbedtls')
})

test('TLS Module - throw on unsupported provider', () => {
  expect(() => new Tls('unknown_provider')).toThrow()
})
