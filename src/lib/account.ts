export class Account {
	app: string;
	url: string;
	username: string;
	password: string;
	id?: number;

	constructor(app: string, url: string, username: string, password: string, id?: number) {
		this.app = app;
		this.url = url;
		this.username = username;
		this.password = password;
		this.id = id;
	}
}
