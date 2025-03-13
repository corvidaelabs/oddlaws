<script lang="ts">
	import { enhance } from '$app/forms';
	import type { ActionData } from './$types';

	let { form }: { form: ActionData } = $props();
</script>

<section class="flex min-h-full w-full grow items-center justify-center">
	<form
		method="post"
		action="?/login"
		use:enhance
		class="card border-surface-200-800 w-full max-w-md space-y-4 border-[1px] p-4"
	>
		<h1>Login/Register</h1>
		<label class="label">
			<span class="label-text">Username</span>
			<input class="input" name="username" />
		</label>
		<label class="label">
			<span class="label-text">Password</span>
			<input class="input" type="password" name="password" />
		</label>
		<button class="btn preset-filled-primary-500">Login</button>
		<button formaction="?/register" class="btn preset-outlined-primary-500">Register</button>
	</form>
	<p style="color: red">{form?.message ?? ''}</p>
</section>
