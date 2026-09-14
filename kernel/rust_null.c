// SPDX-License-Identifier: GPL-2.0
/*
 * rust_null — misc device with the Linux /dev/null I/O contract.
 *
 * /dev/null itself is mem 1:3 (drivers/char/mem.c, null_fops). This
 * module registers a *misc* device named "rust-null" (major 10, dynamic
 * minor) whose file_operations copy that contract:
 *
 *   read_null      → return 0
 *   write_null     → return count (do not touch the user buffer)
 *   null_lseek     → file->f_pos = 0; return 0
 *   read_iter_null / write_iter_null
 *
 * The VFS hook is struct file_operations, hung off struct miscdevice
 * via misc_register(). That is the same path Rust-for-Linux
 * kernel::miscdevice::MiscDevice uses (see rfl/rust_null.rs). Distro
 * kernels without CONFIG_RUST cannot compile the .rs; this .c is the
 * loadable module.
 */

#include <linux/fs.h>
#include <linux/miscdevice.h>
#include <linux/module.h>
#include <linux/uio.h>

static ssize_t rust_null_read(struct file *file, char __user *buf,
			      size_t count, loff_t *ppos)
{
	return 0;
}

static ssize_t rust_null_write(struct file *file, const char __user *buf,
			       size_t count, loff_t *ppos)
{
	return count;
}

static loff_t rust_null_llseek(struct file *file, loff_t offset, int orig)
{
	return file->f_pos = 0;
}

static ssize_t rust_null_read_iter(struct kiocb *iocb, struct iov_iter *to)
{
	return 0;
}

static ssize_t rust_null_write_iter(struct kiocb *iocb, struct iov_iter *from)
{
	size_t count = iov_iter_count(from);

	iov_iter_advance(from, count);
	return count;
}

static const struct file_operations rust_null_fops = {
	.owner		= THIS_MODULE,
	.llseek		= rust_null_llseek,
	.read		= rust_null_read,
	.write		= rust_null_write,
	.read_iter	= rust_null_read_iter,
	.write_iter	= rust_null_write_iter,
};

static struct miscdevice rust_null_misc = {
	.minor = MISC_DYNAMIC_MINOR,
	.name  = "rust-null",
	.fops  = &rust_null_fops,
	.mode  = 0666,
};

static int __init rust_null_init(void)
{
	int err;

	err = misc_register(&rust_null_misc);
	if (err)
		return err;

	pr_info("rust_null: /dev/rust-null registered (misc dynamic minor)\n");
	return 0;
}

static void __exit rust_null_exit(void)
{
	misc_deregister(&rust_null_misc);
	pr_info("rust_null: /dev/rust-null unregistered\n");
}

module_init(rust_null_init);
module_exit(rust_null_exit);

MODULE_LICENSE("GPL");
MODULE_AUTHOR("evie <evie@hareai.dev>");
MODULE_DESCRIPTION("misc device with the /dev/null I/O contract");
MODULE_VERSION("0.1.0");
