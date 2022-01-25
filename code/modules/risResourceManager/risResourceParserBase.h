#pragma once
#include "risResourceBase.h"
#include "risResourceType.h"
#include "../risData/risPrimitives.h"

namespace risResource
{
	using namespace risData;

	class risResourceParserBase
	{
	public:
		virtual ~risResourceParserBase() = default;
		virtual risResourceType get_resource_type() = 0;

		virtual risResourceBase parseFile(U8* data, U32 count) = 0;
		virtual risResourceBase parseData(U8* data, U32 count) = 0;
	};
}
