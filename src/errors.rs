use failure::{ Error, Fail };
use actix  :: { MailboxError, /*prelude::SendError*/ } ;


pub type EkkeResult<T> = Result< T, Error >;


#[ derive( Debug, Fail ) ]
//
pub enum EkkeIoError
{
	#[ fail( display = "Cannot use socket before connecting" ) ]
	//
	UseSocketBeforeConnect,

	#[ fail( display = "Nobody connected to the socket" ) ]
	//
	NoConnectionsReceived,

	#[ fail( display = "Dispatcher: This is an error in ekke. The mailbox of the {} actor cannot keep up with the message flow, or it has been closed to early. If you run into this, please file an issue at https://github.com/najamelan/ekke. Actix Error: {}", _0, _1 ) ]
	//
	ActixMailboxError( String, MailboxError ),

	#[ fail( display = "Dispatcher: This is an error in ekke. The mailbox of the {} actor cannot keep up with the message flow, or it has been closed to early. If you run into this, please file an issue at https://github.com/najamelan/ekke", _0 ) ]
	//
	ActixSendError( String )
}
