// This file is part of rustls-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rustls-extra/master/COPYRIGHT. No part of rustls-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2019 The developers of rustls-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rustls-extra/master/COPYRIGHT.


/// List from <https://www.iana.org/assignments/tls-extensiontype-values/tls-extensiontype-values.xhtml#alpn-protocol-ids> as of January 15th, 2019.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ApplicationLayerProtocolNegotiationProtocol
{
	/// HTTP/0.9 defined in RFC 1945.
	HTTP_0_9,

	/// HTTP/1.0 defined in RFC 1945.
	HTTP_1_0,

	/// HTTP/1.1 defined in RFC 7230.
	HTTP_1_1,

	/// SPDY/1 defined in <http://dev.chromium.org/spdy/spdy-protocol/spdy-protocol-draft1>.
	SPDY_1,

	/// SPDY/2 defined in <http://dev.chromium.org/spdy/spdy-protocol/spdy-protocol-draft2>.
	SPDY_2,

	/// SPDY/3 defined in <http://dev.chromium.org/spdy/spdy-protocol/spdy-protocol-draft3>.
	SPDY_3,

	/// Traversal Using Relays around NAT (TURN) defined in RFC 7443.
	StunTurn,

	/// NAT discovery using Session Traversal Utilities for NAT (STUN) defined in RFC 7443.
	StunNatDiscovery,

	/// HTTP/2 over TLS defined in RFC 7540.
	HTTP_2_over_TLS,

	/// HTTP/2 over TCP defined in RFC 7540.
	///
	/// ***NOTE: This identifier is for use within a cleartext version of a protocol and is not allowed to appear in a TLS ALPN negotiation.***
	HTTP_2_over_TCP,

	/// WebRTC Media and Data defined in RFC-ietf-rtcweb-alpn-04.
	WebRTC,

	/// Confidential WebRTC Media and Data defined in RFC-ietf-rtcweb-alpn-04.
	ConfidentialWebRTC,

	/// FTP defined in RFC 959 and RFC 4217.
	FTP,

	/// IMAP defined in RFC 2595.
	IMAP,

	/// POP3 defined in RFC 2595.
	POP3,

	/// ManageSieve defined in RFC 5804.
	ManageSieve,

	/// CoAP defined in RFC C8323.
	CoAP,

	/// XMPP jabber:client namespace defined in <https://xmpp.org/extensions/xep-0368.html>.
	XMPP_Client,

	/// XMPP jabber:server namespace defined in <https://xmpp.org/extensions/xep-0368.html>.
	XMPP_Server,

	/// Unofficial
	Unofficial(Vec<u8>),
}

impl ApplicationLayerProtocolNegotiationProtocol
{
	/// Converts to a string.
	#[inline(always)]
	pub fn to_vec(&self) -> Vec<u8>
	{
		use self::ApplicationLayerProtocolNegotiationProtocol::*;

		match &self
		{
			HTTP_0_9 => b"http/0.9".to_vec(),
			HTTP_1_0 => b"http/1.0".to_vec(),
			HTTP_1_1 => b"http/1.1".to_vec(),
			SPDY_1 => b"spdy/1".to_vec(),
			SPDY_2 => b"spdy/2".to_vec(),
			SPDY_3 => b"spdy/3".to_vec(),
			StunTurn => b"stun.turn".to_vec(),
			StunNatDiscovery => b"stun.nat-discovery".to_vec(),
			HTTP_2_over_TLS => b"h2".to_vec(),
			HTTP_2_over_TCP => b"h2c".to_vec(),
			WebRTC => b"webrtc".to_vec(),
			ConfidentialWebRTC => b"c-webrtc".to_vec(),
			FTP => b"ftp".to_vec(),
			IMAP => b"imap".to_vec(),
			POP3 => b"pop3".to_vec(),
			ManageSieve => b"managesieve".to_vec(),
			CoAP => b"coap".to_vec(),
			XMPP_Client => b"xmpp-client".to_vec(),
			XMPP_Server => b"xmpp-server".to_vec(),
			Unofficial(ref value) => value.clone(),
		}
	}
}
