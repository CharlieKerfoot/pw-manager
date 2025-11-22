<script lang="ts">
	import Icon from '@iconify/svelte';
	import { Card, SelectModal, SettingsModal, Account, Login } from '$lib';
	import { onMount } from 'svelte';

	let showModal = $state(false);
	let showSettings = $state(false);
	let logged_in = $state(false);

	let cards = $state<Account[]>([]);

	async function fetchAccounts() {
		const token = localStorage.getItem('token');
		if (!token) {
			logged_in = false;
			return;
		}

		try {
			const res = await fetch('http://localhost:3000/accounts', {
				headers: { 'Authorization': `Bearer ${token}` }
			});
			if (res.ok) {
				cards = await res.json();
			} else if (res.status === 401) {
				logged_in = false;
				localStorage.removeItem('token');
			}
		} catch (e) {
			console.error('Failed to fetch accounts', e);
		}
	}

	onMount(() => {
		const token = localStorage.getItem('token');
		if (token) {
			logged_in = true;
			fetchAccounts();
		}
	});

	let selectedAccount = $state<Account | null>(null);

	async function handleSave(account: Account) {
		const token = localStorage.getItem('token');
		if (!token) return;

		try {
			const headers = { 
				'Content-Type': 'application/json',
				'Authorization': `Bearer ${token}`
			};

			if (account.id) {
				await fetch(`http://localhost:3000/accounts/${account.id}`, {
					method: 'PUT',
					headers,
					body: JSON.stringify(account)
				});
			} else {
				await fetch('http://localhost:3000/accounts', {
					method: 'POST',
					headers,
					body: JSON.stringify(account)
				});
			}
			await fetchAccounts();
			selectedAccount = null;
			showModal = false;
		} catch (e) {
			console.error('Failed to save account', e);
		}
	}

	let searchTerm = $state('');

	let filteredCards = $derived(
		cards.filter(
			(card) =>
				card.app.toLowerCase().includes(searchTerm.toLowerCase()) ||
				card.username.toLowerCase().includes(searchTerm.toLowerCase()) ||
				card.url.toLowerCase().includes(searchTerm.toLowerCase())
		)
	);
</script>

{#if !logged_in}
	<Login
		handleSubmit={(email: string, password: string) => {
			logged_in = true;
			fetchAccounts();
		}}
	/>
{:else}
	<div class="marquee-container">
		<div class="marquee">
			<span>THE PASSWORD MANAGER /// SECURE VAULT ACCESS /// ENCRYPTED STORAGE /// </span>
			<span>THE PASSWORD MANAGER /// SECURE VAULT ACCESS /// ENCRYPTED STORAGE /// </span>
		</div>
	</div>

	<div class="header">
		<div class="logo-section">
			<div class="logo-glitch" data-text="TPM">TPM</div>
			<div class="status-indicator">
				<div class="status-dot"></div>
				<span>SYSTEM_ONLINE</span>
			</div>
		</div>
		
		<div class="header-controls">
			<button class="icon-btn" onclick={() => (showSettings = true)} aria-label="Settings">
				[SETTINGS]
			</button>
			<button class="icon-btn logout" onclick={() => {
				logged_in = false;
				localStorage.removeItem('token');
			}} aria-label="Logout">
				[DISCONNECT]
			</button>
		</div>
	</div>

	<div class="controls">
		<div class="search-bar">
			<span class="search-icon">>></span>
			<input type="text" placeholder="SEARCH_DATABASE..." bind:value={searchTerm} />
		</div>
		<button
			class="add-btn"
			onclick={() => {
				selectedAccount = null;
				showModal = true;
			}}
		>
			+ NEW_ENTRY
		</button>
	</div>

	<div class="main-div">
		{#each filteredCards as card}
			<Card
				{...card}
				onDelete={async () => {
					if (!confirm('CONFIRM_DELETION: THIS ACTION IS IRREVERSIBLE.')) return;

					const token = localStorage.getItem('token');
					if (!token) return;

					const originalCards = cards;
					cards = cards.filter((c) => c.id !== card.id);

					try {
						await fetch(`http://localhost:3000/accounts/${card.id}`, {
							method: 'DELETE',
							headers: { 'Authorization': `Bearer ${token}` }
						});
					} catch (e) {
						console.error('Failed to delete account', e);
						cards = originalCards;
					}
				}}
				onEdit={() => {
					selectedAccount = card;
					showModal = true;
				}}
			/>
		{/each}
	</div>
{/if}

<SelectModal bind:showModal onSave={handleSave} account={selectedAccount} />
<SettingsModal bind:showSettings onClearData={fetchAccounts} />

<style>
	/* Marquee */
	.marquee-container {
		background: var(--color-accent);
		color: black;
		overflow: hidden;
		white-space: nowrap;
		padding: 0.5em 0;
		font-family: var(--font-mono);
		font-weight: 700;
		border-bottom: 2px solid black;
	}

	.marquee {
		display: inline-block;
		animation: scroll 20s linear infinite;
	}

	@keyframes scroll {
		0% { transform: translateX(0); }
		100% { transform: translateX(-50%); }
	}

	/* Header */
	.header {
		display: flex;
		justify-content: space-between;
		align-items: flex-end;
		padding: 4em 4em 2em 4em;
		border-bottom: 2px solid var(--color-border);
		background: var(--color-bg);
	}

	.logo-glitch {
		font-size: 6em;
		font-weight: 900;
		line-height: 0.8;
		color: white;
		position: relative;
		font-family: var(--font-header);
	}

	.logo-glitch::before {
		content: attr(data-text);
		position: absolute;
		left: 2px;
		text-shadow: -1px 0 red;
		top: 0;
		color: white;
		background: var(--color-bg);
		overflow: hidden;
		clip: rect(0, 900px, 0, 0);
		animation: noise-anim-2 3s infinite linear alternate-reverse;
	}

	@keyframes noise-anim-2 {
		0% { clip: rect(12px, 9999px, 52px, 0); }
		5% { clip: rect(32px, 9999px, 12px, 0); }
		10% { clip: rect(82px, 9999px, 92px, 0); }
		100% { clip: rect(12px, 9999px, 2px, 0); }
	}

	.status-indicator {
		display: flex;
		align-items: center;
		gap: 0.5em;
		margin-top: 1em;
		font-size: 0.8em;
		color: var(--color-accent);
	}

	.status-dot {
		width: 8px;
		height: 8px;
		background: var(--color-accent);
		border-radius: 50%;
		box-shadow: 0 0 10px var(--color-accent);
	}

	.header-controls {
		display: flex;
		gap: 1em;
	}

	.icon-btn {
		background: transparent;
		border: 1px solid var(--color-border);
		color: #666;
		padding: 0.5em 1em;
		font-family: var(--font-mono);
		font-size: 0.9em;
		transition: all 0.2s;
	}

	.icon-btn:hover {
		color: white;
		border-color: white;
	}

	.icon-btn.logout:hover {
		color: var(--color-danger);
		border-color: var(--color-danger);
	}

	/* Controls */
	.controls {
		display: flex;
		justify-content: space-between;
		padding: 2em 4em;
		align-items: center;
	}

	.search-bar {
		display: flex;
		align-items: center;
		gap: 1em;
		border-bottom: 2px solid var(--color-border);
		padding: 0.5em 0;
		width: 40%;
		transition: border-color 0.2s;
	}

	.search-bar:focus-within {
		border-color: var(--color-accent);
	}

	.search-icon {
		color: var(--color-accent);
		font-weight: 700;
	}

	.search-bar input {
		background: transparent;
		border: none;
		color: white;
		width: 100%;
		font-size: 1.2em;
		text-transform: uppercase;
		padding: 0;
	}

	.search-bar input:focus {
		outline: none;
		box-shadow: none;
	}

	.add-btn {
		background: var(--color-surface);
		color: white;
		border: 1px solid var(--color-border);
		padding: 1em 2em;
		font-size: 1em;
		transition: all 0.2s;
	}

	.add-btn:hover {
		background: var(--color-accent);
		color: black;
		border-color: var(--color-accent);
		box-shadow: 4px 4px 0px white;
		transform: translate(-2px, -2px);
	}

	.main-div {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 2em;
		padding: 0 4em 4em 4em;
	}
</style>
