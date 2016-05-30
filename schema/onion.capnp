@0xfd43b6dc99c3e4f6;

enum OnionCommand {
	newConnection @0;
	newChannel @1;
}

struct OnionMessage {
    command @0 :OnionCommand;
}
