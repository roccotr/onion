
use capnp;
//use capnp::message::MessageReader;



pub struct OnionMessageReader<T> where T: for<'a> ::capnp::traits::Owned<'a> {
    message: ::capnp::message::Reader<::capnp::serialize::OwnedSegments>,
    phantom_data: ::std::marker::PhantomData<T>,
}

impl <T> OnionMessageReader<T>  where T: for<'a> ::capnp::traits::Owned<'a>{

    fn new(message: ::capnp::message::Reader<::capnp::serialize::OwnedSegments>) -> OnionMessageReader<T> {
        OnionMessageReader{
            phantom_data: ::std::marker::PhantomData,
            message: message
        }
    }

    pub fn new_from_buffer(mut buffer: &[u8]) -> capnp::Result<OnionMessageReader<T>> {
        let mut message_reader = try!(capnp::serialize::read_message(&mut buffer, ::capnp::message::ReaderOptions::new()));
        Ok(OnionMessageReader::new(message_reader))

    }
}

impl <T> OnionMessageReader <T>  where T: for<'a> ::capnp::traits::Owned<'a>{
    fn get<'a>(&'a self) -> ::capnp::Result<<T as ::capnp::traits::Owned<'a>>::Reader > {
        self.message.get_root()
    }
}
//
//     pub fn get_command<'a>(mut command: &[u8]) -> capnp::Result<::onion_capnp::onion_message::Reader<'a>> {
//         let mut message_reader = try!(capnp::serialize::read_message(&mut command, ::capnp::message::ReaderOptions::new()));
//         message_reader.get_root::<::onion_capnp::onion_message::Reader>()
//     }
// }

pub struct OnionMessageWriter;

impl OnionMessageWriter {
    pub fn new() -> Result<Vec<u8>,String> {
        let mut message = ::capnp::message::Builder::new_default();
        {
            let mut command = message.init_root::<::onion_capnp::onion_message::Builder>();
            command.set_command(::onion_capnp::OnionCommand::NewConnection);
        }
        let mut serialized_message: Vec<u8> = Vec::new();
        capnp::serialize::write_message(&mut serialized_message, &message);
        Ok(serialized_message)
    }
}
