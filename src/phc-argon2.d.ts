declare module '@phc/argon2' {
	const argon2: {
		hash: (options: {
			password: string | Uint8Array;
			salt?: string | Uint8Array;
			algorithm?: 'argon2id' | 'argon2i' | 'argon2d';
			memory?: number;
			iterations?: number;
			parallelism?: number;
			hashLength?: number;
			type?: string;
		}) => Promise<string>;

		verify: (options: {
			hash: string;
			password: string | Uint8Array;
		}) => Promise<boolean>;
	};

	export default argon2;
}
