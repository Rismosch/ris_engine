#pragma once

namespace risEngine
{
	class risJobSystem
	{
	public:
		// singleton policy
		static risJobSystem* instance();
		static void create();
		static void destroy();

		risJobSystem(const risJobSystem& other) = delete;
		risJobSystem(risJobSystem && other) noexcept = delete;
	private:
		risJobSystem() = default;
		static risJobSystem* p_instance_;
	};
}
