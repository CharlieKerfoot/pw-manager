<script lang="ts">
	import { Account } from './account';

	let { showModal = $bindable(), onSave, account } = $props();

	let app = $state('');
	let url = $state('');
	let username = $state('');
	let password = $state('');

	$effect(() => {
		if (account) {
			app = account.app;
			url = account.url;
			username = account.username;
			password = account.password;
		} else {
			app = '';
			url = '';
			username = '';
			password = '';
		}
	});

	function handleSave() {
		const newAccount = new Account(app, url, username, password, account?.id);
		onSave(newAccount);
	}
</script>

<dialog open={showModal}>
	<div class="modal-content">
		<div class="modal-header">
			<h2>{account ? 'EDIT_ENTRY' : 'NEW_ENTRY'}</h2>
			<button class="close-btn" onclick={() => (showModal = false)}>[X]</button>
		</div>
		
		<div class="form-grid">
			<div class="form-group full-width">
				<label for="app">APPLICATION_ID</label>
				<input type="text" id="app" bind:value={app} placeholder="ENTER_APP_NAME" />
			</div>
			
			<div class="form-group full-width">
				<label for="url">TARGET_URL</label>
				<input type="text" id="url" bind:value={url} placeholder="HTTPS://" />
			</div>
			
			<div class="form-group">
				<label for="username">USER_ID</label>
				<input type="text" id="username" bind:value={username} placeholder="USERNAME" />
			</div>
			
			<div class="form-group">
				<label for="password">ACCESS_KEY</label>
				<input type="text" id="password" bind:value={password} placeholder="PASSWORD" />
			</div>
		</div>

		<div class="modal-actions">
			<button class="cancel-btn" onclick={() => (showModal = false)}>ABORT</button>
			<button class="save-btn" onclick={handleSave}>
				{account ? 'UPDATE_RECORD' : 'INIT_RECORD'}
			</button>
		</div>
	</div>
</dialog>

<style>
	dialog {
		background: rgba(0, 0, 0, 0.8);
		border: none;
		width: 100%;
		height: 100%;
		position: fixed;
		top: 0;
		left: 0;
		display: none;
		justify-content: center;
		align-items: center;
		z-index: 1000;
		padding: 0;
		margin: 0;
	}

	dialog[open] {
		display: flex;
	}

	.modal-content {
		background: var(--color-bg);
		border: 2px solid var(--color-accent);
		padding: 2em;
		width: 100%;
		max-width: 500px;
		box-shadow: var(--shadow-hard);
		position: relative;
	}

	.modal-header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2em;
		border-bottom: 1px solid var(--color-border);
		padding-bottom: 1em;
	}

	h2 {
		color: var(--color-accent);
		font-size: 1.5em;
		margin: 0;
	}

	.close-btn {
		background: transparent;
		border: none;
		color: #666;
		font-size: 1.2em;
		padding: 0;
	}

	.close-btn:hover {
		color: var(--color-danger);
	}

	.form-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1.5em;
		margin-bottom: 2em;
	}

	.full-width {
		grid-column: span 2;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.5em;
	}

	label {
		color: #666;
		font-size: 0.8em;
		font-weight: 700;
	}
	input {
		padding: 0.8em;
		background: #222;
		border: 1px solid #333;
		border-radius: 4px;
		color: white;
		font-family: var(--font-mono);
		font-size: 0.9em;
		transition: border-color 0.2s;
	}
	input:focus {
		outline: none;
		border-color: #666;
	}
	.save-btn {
		margin-top: 1em;
		padding: 1em;
		background: white;
		color: black;
		border: none;
		border-radius: 4px;
		font-weight: 600;
		cursor: pointer;
		transition: opacity 0.2s;
	}
	.save-btn:hover {
		opacity: 0.9;
	}

</style>
