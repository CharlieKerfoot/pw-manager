<script lang="ts">
	let { handleSubmit } = $props();
	let email = $state('');
	let password = $state('');
	let isRegistering = $state(false);
	let error = $state('');

	async function onSubmit(e: Event) {
		e.preventDefault();
		error = '';

		const endpoint = isRegistering ? '/register' : '/login';
		const payload = { email, password_hash: password };

		try {
			const res = await fetch(`${import.meta.env.VITE_API_URL}${endpoint}`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(payload)
			});

			if (res.ok) {
				if (isRegistering) {
					const loginRes = await fetch(`${import.meta.env.VITE_API_URL}/login`, {
						method: 'POST',
						headers: { 'Content-Type': 'application/json' },
						body: JSON.stringify(payload)
					});
					if (loginRes.ok) {
						const data = await loginRes.json();
						localStorage.setItem('token', data.token);
						handleSubmit(email, password);
					} else {
						error = 'REGISTRATION_FAILED'; // Or handle login failure after registration
					}
				} else {
					const data = await res.json();
					localStorage.setItem('token', data.token);
					handleSubmit(email, password);
				}
			} else {
				error = isRegistering ? 'REGISTRATION_FAILED' : 'ACCESS_DENIED';
			}
		} catch (e) {
			error = 'SYSTEM_ERROR';
			console.error(e);
		}
	}
</script>

<div class="login-container">
	<div class="scan-line"></div>
	<div class="login-box">
		<div class="header-glitch" data-text={isRegistering ? 'INIT_PROTOCOL: NEW_USER' : 'SYSTEM_ACCESS: REQUIRED'}>
			{isRegistering ? 'INIT_PROTOCOL: NEW_USER' : 'SYSTEM_ACCESS: REQUIRED'}
		</div>
		
		<form onsubmit={onSubmit}>
			<div class="form-group">
				<label for="email">USER_ID</label>
				<input type="email" id="email" bind:value={email} required placeholder="ENTER_EMAIL" />
			</div>
			<div class="form-group">
				<label for="password">ACCESS_CODE</label>
				<input type="password" id="password" bind:value={password} required placeholder="••••••••" />
			</div>
			{#if error}
				<div class="error">ERROR: {error}</div>
			{/if}
			<button type="submit">
				{isRegistering ? 'INITIALIZE_USER' : 'AUTHENTICATE'}
			</button>
		</form>

		<button class="toggle-btn" onclick={() => (isRegistering = !isRegistering)}>
			[{isRegistering ? 'SWITCH_TO_LOGIN' : 'SWITCH_TO_REGISTER'}]
		</button>
	</div>
</div>

<style>
	.login-container {
		display: flex;
		justify-content: center;
		align-items: center;
		height: 100vh;
		background: var(--color-bg);
		position: relative;
		overflow: hidden;
	}

	/* Scan line effect */
	.scan-line {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 2px;
		background: rgba(204, 255, 0, 0.3);
		animation: scan 3s linear infinite;
		pointer-events: none;
		z-index: 10;
	}

	@keyframes scan {
		0% { top: 0%; }
		100% { top: 100%; }
	}

	.login-box {
		background: var(--color-bg);
		padding: 3em;
		border: 2px solid var(--color-border);
		width: 100%;
		max-width: 450px;
		position: relative;
	}

	.login-box::before {
		content: '';
		position: absolute;
		top: -10px;
		left: -10px;
		width: 20px;
		height: 20px;
		border-top: 4px solid var(--color-accent);
		border-left: 4px solid var(--color-accent);
	}

	.login-box::after {
		content: '';
		position: absolute;
		bottom: -10px;
		right: -10px;
		width: 20px;
		height: 20px;
		border-bottom: 4px solid var(--color-accent);
		border-right: 4px solid var(--color-accent);
	}

	.header-glitch {
		font-family: var(--font-header);
		font-size: 1.5em;
		color: var(--color-text);
		margin-bottom: 1.5em;
		text-align: center;
		position: relative;
	}

	form {
		display: flex;
		flex-direction: column;
		gap: 2em;
	}

	.form-group {
		display: flex;
		flex-direction: column;
		gap: 0.5em;
	}

	label {
		color: var(--color-accent);
		font-size: 0.8em;
		font-weight: 700;
	}

	input {
		width: 100%;
		box-sizing: border-box;
		background: transparent;
		border: 1px solid var(--color-border);
		color: white;
		padding: 1em;
		font-size: 1em;
		border-radius: 0;
		font-family: var(--font-mono);
	}

	input:focus {
		border-color: var(--color-accent);
		outline: none;
		box-shadow: 0 0 10px rgba(204, 255, 0, 0.1);
	}

	button[type="submit"] {
		background: var(--color-accent);
		color: black;
		padding: 1.2em;
		border: none;
		font-weight: 900;
		letter-spacing: 1px;
		margin-top: 1em;
		transition: transform 0.1s;
		cursor: pointer;
		font-family: var(--font-mono);
	}

	button[type="submit"]:hover {
		transform: translate(-2px, -2px);
		box-shadow: 4px 4px 0px white;
	}

	button[type="submit"]:active {
		transform: translate(0, 0);
		box-shadow: none;
	}

	.toggle-btn {
		background: none;
		border: none;
		color: #666;
		font-size: 0.8em;
		margin-top: 2em;
		width: 100%;
		text-align: center;
		cursor: pointer;
		font-family: var(--font-mono);
	}

	.toggle-btn:hover {
		color: var(--color-accent);
	}

	.error {
		color: var(--color-danger);
		border: 1px solid var(--color-danger);
		padding: 0.5em;
		text-align: center;
		font-size: 0.9em;
		background: rgba(255, 51, 0, 0.1);
		font-family: var(--font-mono);
	}
</style>
