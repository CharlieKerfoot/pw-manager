<script lang="ts">
	import { Account } from './account';
	import Icon from '@iconify/svelte';

	let { app, url, username, password, onDelete, onEdit } = $props();

	let showPassword = $state(false);

	function copyToClipboard(text: string) {
		navigator.clipboard.writeText(text);
	}
</script>

<div class="card">
	<div class="card-header">
		<div class="app-name">{app}</div>
		<div class="card-actions">
			<button class="action-btn edit" onclick={onEdit} aria-label="Edit">
				<Icon icon="material-symbols:edit-outline" />
			</button>
			<button class="action-btn delete" onclick={onDelete} aria-label="Delete">
				<Icon icon="material-symbols:delete-outline" />
			</button>
		</div>
	</div>

	<div class="card-body">
		<div class="field">
			<label>URL_TARGET</label>
			<div class="value-row">
				<a href={url} target="_blank" class="url-link">{url}</a>
				<button class="copy-btn" onclick={() => copyToClipboard(url)}>CPY</button>
			</div>
		</div>

		<div class="field">
			<label>USER_ID</label>
			<div class="value-row">
				<span class="value">{username}</span>
				<button class="copy-btn" onclick={() => copyToClipboard(username)}>CPY</button>
			</div>
		</div>

		<div class="field">
			<label>ACCESS_KEY</label>
			<div class="value-row">
				<span class="value password">{showPassword ? password : '••••••••••••'}</span>
				<div class="pw-controls">
					<button class="copy-btn" onclick={() => copyToClipboard(password)}>CPY</button>
					<button class="toggle-btn" onclick={() => (showPassword = !showPassword)}>
						{showPassword ? 'HIDE' : 'SHOW'}
					</button>
				</div>
			</div>
		</div>
	</div>
</div>

<style>
	.card {
		background: var(--color-surface);
		border: 1px solid var(--color-border);
		padding: 0;
		position: relative;
		transition: all 0.2s ease;
	}

	.card:hover {
		border-color: var(--color-accent);
		transform: translateY(-4px);
		box-shadow: var(--shadow-hard);
	}

	.card-header {
		background: var(--color-bg);
		border-bottom: 1px solid var(--color-border);
		padding: 1em;
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.app-name {
		font-family: var(--font-header);
		font-size: 1.2em;
		color: white;
	}

	.card:hover .app-name {
		color: var(--color-accent);
	}

	.card-actions {
		display: flex;
		gap: 0.5em;
		opacity: 0;
		transition: opacity 0.2s;
	}

	.card:hover .card-actions {
		opacity: 1;
	}

	.action-btn {
		background: transparent;
		border: none;
		color: #666;
		padding: 0.2em;
	}

	.action-btn:hover {
		color: white;
	}

	.action-btn.delete:hover {
		color: var(--color-danger);
	}

	.card-body {
		padding: 1.5em;
		display: flex;
		flex-direction: column;
		gap: 1.5em;
	}

	.field {
		display: flex;
		flex-direction: column;
		gap: 0.5em;
	}

	label {
		font-size: 0.7em;
		color: #666;
		font-weight: 700;
	}

	.value-row {
		display: flex;
		justify-content: space-between;
		align-items: center;
		background: rgba(255, 255, 255, 0.03);
		padding: 0.5em;
		border: 1px solid transparent;
	}

	.value-row:hover {
		border-color: #333;
	}

	.value {
		font-family: var(--font-mono);
		font-size: 0.9em;
		color: #ddd;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.url-link {
		color: var(--color-accent);
		text-decoration: none;
		font-size: 0.9em;
	}

	.url-link:hover {
		text-decoration: underline;
		color: #000;
	}
</style>
